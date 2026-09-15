{
  pkgs,
  lib,
  microvm,
  osmium,
  sceptre,
}:

let
  adminPassword = "sceptre-test-admin-password";
  userPassword = "sceptre-test-user-password";
  owner = "sceptre-user";
  grimoireName = "sceptre-grimoire";
  templateName = "sceptre-template";
  targetName = "created-fixture";
  giteaHost = "gitea.test";
  giteaUrl = "http://${giteaHost}:3000";

  seedGrimoire = pkgs.runCommand "sceptre-seed-grimoire" { } ''
    mkdir -p "$out/repos"
    printf '%s\n' \
      '{' \
      '  "id": "created-fixture",' \
      '  "name": "Created Fixture",' \
      '  "upstream": "http://gitea.test:3000/sceptre-user/created-fixture",' \
      '  "local_path": "/tmp/sceptre-created-fixture",' \
      '  "template": "fixture-template",' \
      '  "upstream_created": false,' \
      '  "default_branch": "main",' \
      '  "role": "test-fixture"' \
      '}' > "$out/repos/${targetName}.json"
    printf '%s\n' \
      'templates:' \
      '  - id: fixture-template' \
      '    name: Fixture Template' \
      '    repository: sceptre-user/sceptre-template' \
      '    upstream: http://gitea.test:3000/sceptre-user/sceptre-template' \
      > "$out/repos/templates.yaml"
    printf '%s\n' '# Sceptre Gitea repository-creation fixture' > "$out/README.md"
  '';

  seedTemplate = pkgs.runCommand "sceptre-seed-template" { } ''
    mkdir -p "$out"
    printf '%s\n' 'created from the Sceptre integration-test template' > "$out/template-marker.txt"
    printf '%s\n' '# Sceptre template fixture' > "$out/README.md"
  '';

  testModule = { config, pkgs, ... }: {
    imports = [
      microvm.nixosModules.microvm
      osmium.nixosModules.default
    ];

    system.stateVersion = "25.05";
    nixpkgs.overlays = lib.mkForce [ ];
    networking.hostName = "sceptre-repository-creation";
    networking.extraHosts = "127.0.0.1 ${giteaHost}";
    networking.firewall.allowedTCPPorts = [ 3000 ];

    microvm = {
      hypervisor = "qemu";
      vcpu = 2;
      mem = 1536;
      interfaces = [
        {
          type = "user";
          id = "sceptre-create";
          mac = "02:00:00:00:00:0e";
        }
      ];
    };

    virtualisation.graphics = false;
    virtualisation.diskSize = 4096;

    environment.systemPackages = with pkgs; [
      curl
      git
      jq
      tea
      sceptre
    ];
    environment.etc."gitea-admin-password" = {
      text = "${adminPassword}\n";
      mode = "0400";
      user = "gitea";
      group = "gitea";
    };
    environment.etc."sceptre-test-user-password" = {
      text = "${userPassword}\n";
      mode = "0400";
      user = "gitea";
      group = "gitea";
    };
    environment.etc."sceptre-seed-grimoire".source = seedGrimoire;
    environment.etc."sceptre-seed-template".source = seedTemplate;

    services.osmium.gitea = {
      enable = true;
      hostHttpPort = 3001;
      hostSshPort = 2224;
      settings.server.START_SSH_SERVER = true;
      settings.service.DISABLE_REGISTRATION = false;
      admin = {
        enable = true;
        username = "sceptre-admin";
        email = "sceptre-admin@example.invalid";
        passwordFile = "/etc/gitea-admin-password";
      };
      users.sceptre = {
        username = owner;
        email = "sceptre-user@example.invalid";
        passwordFile = "/etc/sceptre-test-user-password";
      };
      repositories.grimoire = {
        owner.user = owner;
        name = grimoireName;
        description = "Sceptre Grimoire fixture";
        private = false;
        defaultBranch = "main";
        issues = false;
        wiki = false;
        pullRequests = false;
      };
      repositories.template = {
        owner.user = owner;
        name = templateName;
        description = "Sceptre template fixture";
        private = false;
        defaultBranch = "main";
        issues = false;
        wiki = false;
        pullRequests = false;
      };
      credentials.sceptre = {
        kind = "personal-token";
        username = owner;
        scopes = [
          "read:user"
          "read:issue"
          "write:repository"
        ];
        output = {
          secretPath = "/var/lib/gitea/credentials/sceptre.token";
          owner = "root";
          group = "root";
          mode = "0400";
        };
      };
    };
  };
in
pkgs.testers.runNixOSTest {
  name = "sceptre-repository-creation-gitea";
  nodes.vm = testModule;
  testScript = ''
    vm.start(allow_reboot=True)
    vm.wait_for_unit("gitea.service")
    vm.wait_for_unit("osmium-gitea-admin-bootstrap.service")
    vm.wait_until_succeeds("test -s /var/lib/gitea/credentials/sceptre.token")
    vm.succeed("curl --fail http://${giteaHost}:3000/api/healthz")
    vm.succeed("test \"$(stat -c '%a' /var/lib/gitea/credentials/sceptre.token)\" = 400")
    vm.succeed("git config --global user.name sceptre-test")
    vm.succeed("git config --global user.email sceptre-test@example.invalid")
    vm.succeed("mkdir -p /root/.config/tea; tea logins add --url ${giteaUrl} --user ${owner} --password ${userPassword} --name local --git-credentials")
    vm.succeed("token=$(cat /var/lib/gitea/credentials/sceptre.token); rm -rf /tmp/grimoire-seed; cp -aL /etc/sceptre-seed-grimoire /tmp/grimoire-seed; cd /tmp/grimoire-seed; git init -b main; git add .; git commit -m seed; git remote add origin http://${owner}:$token@${giteaHost}:3000/${owner}/${grimoireName}.git; git push origin main")
    vm.succeed("token=$(cat /var/lib/gitea/credentials/sceptre.token); rm -rf /tmp/template-seed; cp -aL /etc/sceptre-seed-template /tmp/template-seed; cd /tmp/template-seed; git init -b main; git add .; git commit -m seed; git remote add origin http://${owner}:$token@${giteaHost}:3000/${owner}/${templateName}.git; git push origin main")
    vm.succeed("token=$(cat /var/lib/gitea/credentials/sceptre.token); ! curl --fail --silent -H \"Authorization: token $token\" ${giteaUrl}/api/v1/repos/${owner}/${targetName} >/dev/null")
    vm.succeed("rm -rf /tmp/grimoire; token=$(cat /var/lib/gitea/credentials/sceptre.token); git clone http://${owner}:$token@${giteaHost}:3000/${owner}/${grimoireName}.git /tmp/grimoire")
    vm.succeed("jq -e . /tmp/grimoire/repos/${targetName}.json")
    vm.succeed("cd /tmp/grimoire && SCEPTRE_GIT=/run/current-system/sw/bin/git ${sceptre}/bin/rust-template repository create --definition repos/${targetName}.json --templates repos/templates.yaml --https --json")
    vm.succeed("token=$(cat /var/lib/gitea/credentials/sceptre.token); curl --fail --silent -H \"Authorization: token $token\" ${giteaUrl}/api/v1/repos/${owner}/${targetName} | jq -e '.name == \"${targetName}\" and .default_branch == \"main\"'")
    vm.succeed("rm -rf /tmp/created-fixture; token=$(cat /var/lib/gitea/credentials/sceptre.token); git clone --branch main http://${owner}:$token@${giteaHost}:3000/${owner}/${targetName}.git /tmp/created-fixture; test \"$(cat /tmp/created-fixture/template-marker.txt)\" = 'created from the Sceptre integration-test template'")
    vm.succeed("rm -rf /tmp/grimoire-verify; token=$(cat /var/lib/gitea/credentials/sceptre.token); git clone --branch main http://${owner}:$token@${giteaHost}:3000/${owner}/${grimoireName}.git /tmp/grimoire-verify; jq -e '.id == \"${targetName}\" and .upstream_created == true and .upstream == \"http://${giteaHost}:3000/${owner}/${targetName}\"' /tmp/grimoire-verify/repos/${targetName}.json; test \"$(git -C /tmp/grimoire-verify log -1 --format=%s)\" = 'Mark ${targetName} upstream as created'")
    vm.shutdown()
  '';
}

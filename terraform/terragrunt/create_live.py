import os
from pathlib import Path

ENVS = ["prod", "staging"]
BASE_DIR = Path(__file__).absolute().parent
MODULES_DIR = Path(BASE_DIR) / "modules"
LIVE_DIR = Path(BASE_DIR) / "live"

for env_name in ENVS:
    env_dir = LIVE_DIR / env_name
    env_dir.mkdir(exist_ok=True)
    print(f"Created {env_dir}")
    for module_name in os.listdir(MODULES_DIR):
        module_dir = env_dir / module_name
        module_dir.mkdir(exist_ok=True)
        print(f"Created {module_dir}")
        (module_dir / "terragrunt.hcl").touch()
        print("Created terragrunt.hcl")
        print("----------------------")

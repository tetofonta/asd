import math
import os
import time
import re

import yaml
from tqdm import tqdm
import subprocess
import matplotlib.pyplot as plt
import numpy

INSTANCE_GEN = './target/profiling/instance_gen'

# test iniziale: generazione con numero fisso di ostacoli
instance_gen_const_obj_mem_result = []

#generate instances
os.makedirs("tests/instance_gen/const_obj_no_agents_obs_incr/", exist_ok=True)
for obs in range(1000, 10000, 250):
    for i in range(25):
        instance = {
            "id": f"const_obj_no_agents_obs_incr_{obs}_{i}",
            "kind": "settings",
            "seed": i,
            "greedy": True,
            "obstacles": obs,
            "time_max": 1,
            "size": {
                "width": math.ceil(math.sqrt(obs)),
                "height": math.ceil(math.sqrt(obs))
            },
            "agents": {"number": 0, "stop_probability": 1}
        }
        y = yaml.dump(instance)
        with open(f"tests/instance_gen/const_obj_no_agents_obs_incr/const_obj_no_agents_obs_incr_{obs}_{i}.yaml", 'w') as out:
            out.write(y)
print("Generated settings for const_obj_no_agents_obs_incr")

for obs in tqdm(range(1000, 10000, 250)):
    instance_gen_const_obj_mem_result.append([])
    for i in tqdm(range(25)):
        settings_name = f"const_obj_no_agents_obs_incr_{obs}_{i}"
        # run instance_gen
        subprocess.run([
            "heaptrack",
            '-o', f"tests/instance_gen/const_obj_no_agents_obs_incr/{settings_name}.heaptrack",
            INSTANCE_GEN,
            '-c', f"tests/instance_gen/const_obj_no_agents_obs_incr/{settings_name}.yaml"
        ], capture_output=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

        data = subprocess.run(['heaptrack_print', f"tests/instance_gen/const_obj_no_agents_obs_incr/{settings_name}.heaptrack.zst"], capture_output=True)
        mem = re.search(r'peak heap memory consumption: ([0-9\\.]+)([KMG]?)', data.stdout.decode().split('\n')[-4])
        instance_gen_const_obj_mem_result[-1].append(float(mem.groups()[0]) * (1 if mem.groups()[1] == 'K' else 1024 if mem.groups()[1] == 'M' else 1/1024))

print(instance_gen_const_obj_mem_result)

plt.boxplot(instance_gen_const_obj_mem_result)
plt.xticks(list(range(1, 37)), list(range(1000, 10000, 250)), rotation=45)
plt.xlabel('Number of obstacles')
plt.ylabel('Allocated memory [KB]')
plt.title('Instance Generation variable number of obstacles')
plt.savefig('grafici/instance_gen_obj_incr.png')

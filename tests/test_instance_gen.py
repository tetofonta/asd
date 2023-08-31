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
instance_gen_const_obj_time_result = []
instance_gen_const_obj_mem_result = []

#generate instances
os.makedirs("tests/instance_gen/const_obj_no_agents_dim_incr/", exist_ok=True)
for dim in range(500, 5100, 100):
    for i in range(100):
        instance = {
            "id": f"const_obj_no_agents_dim_incr_{dim}_{i}",
            "kind": "settings",
            "seed": i,
            "greedy": True,
            "obstacles": 800,
            "time_max": dim*2,
            "size": {
                "width": dim,
                "height": dim
            },
            "agents": {"number": 0, "stop_probability": 1},
            "noise": {
                "cell_size": 5,
                "offset": 0.1
            }
        }
        y = yaml.dump(instance)
        with open(f"tests/instance_gen/const_obj_no_agents_dim_incr/const_obj_no_agents_dim_incr_{dim}_{i}.yaml", 'w') as out:
            out.write(y)
print("Generated settings for const_obj_no_agents_dim_incr")

for dim in tqdm(range(500, 5100, 100)):
    instance_gen_const_obj_time_result.append([])
    instance_gen_const_obj_mem_result.append([])
    for i in tqdm(range(100)):
        settings_name = f"const_obj_no_agents_dim_incr_{dim}_{i}"
        # run instance_gen

        t = time.monotonic_ns()
        subprocess.run([
            "heaptrack",
            '-o', f"tests/instance_gen/const_obj_no_agents_dim_incr/{settings_name}.heaptrack",
            INSTANCE_GEN,
            '-c', f"tests/instance_gen/const_obj_no_agents_dim_incr/{settings_name}.yaml"
        ], capture_output=False, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        duration = time.monotonic_ns() - t
        instance_gen_const_obj_time_result[-1].append(duration)

        data = subprocess.run(['heaptrack_print', f"tests/instance_gen/const_obj_no_agents_dim_incr/{settings_name}.heaptrack.zst"], capture_output=True)
        mem = re.search(r'peak heap memory consumption: ([0-9\\.]+)([KMG]?)', data.stdout.decode().split('\n')[-4])
        instance_gen_const_obj_mem_result[-1].append(float(mem.groups()[0]) * (1024 if mem.groups()[1] == 'K' else 1024*1024 if mem.groups()[1] == 'M' else 1))

print(instance_gen_const_obj_time_result)
print(instance_gen_const_obj_mem_result)


plt.boxplot(instance_gen_const_obj_time_result[5::2],  patch_artist=True, notch=True)
plt.xticks(list(range(1, 24)), list(range(500, 5100, 200)), rotation=45)
plt.xlabel('Instance dimensions')
plt.ylabel('Time took [ns]')
plt.title('Instance Generation, constant objects (800)')
plt.savefig('grafici/instance_gen_800_time.png')

plt.cla()
plt.boxplot(instance_gen_const_obj_mem_result[5::2])
plt.xticks(list(range(1, 24)), list(range(500, 5100, 200)), rotation=45)
plt.xlabel('Instance dimensions')
plt.ylabel('Allocated memory [B]')
plt.title('Instance Generation, constant objects (800)')
plt.savefig('grafici/instance_gen_800_mem.png')

import os
import shutil
import random

N = 1000

bits = 64 * 500 * N

gb = bits / 858993459

print(f"Estimated size (GB): {gb}")

# add an are you sure? here.

# if data directory doesn't exist, create it
if os.path.exists(os.getcwd() + "/data"):
    # if data directory contains files, delete them
    print("Removing existing data.")
    shutil.rmtree(os.getcwd() + "/data")
    os.makedirs(os.getcwd() + "/data")
else:
    os.makedirs(os.getcwd() + "/data")


print(f"Writing example data for {N} users.")
for i in range(1, N):

    n_friends = 100
    sample_range = [i, (i + n_friends) % N]

    friends = random.sample(range(min(sample_range), max(sample_range)),
                            n_friends)

    with open(os.getcwd() + "/data/" + str(i) + ".csv", "w") as f:

        f.write("\n".join([str(x) for x in friends]))

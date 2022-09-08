import subprocess

def input_gen(nums, k):
    return str(k) + '|' + '|'.join([' '.join([str(x) for x in n_list]) for n_list in nums])

def yens(input_values, binary_path='../target/release/yens', k=40):
    p = subprocess.Popen([binary_path], stdin=subprocess.PIPE, stdout=subprocess.PIPE, shell=True)

    out, err = p.communicate(input_gen(input_values, k).encode())

    for line_out in out.decode('utf-8').split('\n'):
        if len(line_out) != 0:
            out_split = line_out.split(' ')

            path = [int(x) for x in out_split[:-1]]
            cost = float(out_split[-1].split('=')[-1])

            yield (path, cost)

import numpy as np
from tqdm import tqdm
import pandas as pd
import time
import multiprocessing
import subprocess
import json
import datetime


def run(i, params=None, tuning=False):
    if tuning:
        args = format(f'{params[0]} {params[1]} {params[2]}')
        output_str = subprocess.run(f'powershell cat in/{i:04}.txt | .\\target\\debug\\ahc057.exe {args} > out/{i:04}.txt', shell=True, capture_output=True, text=True).stderr
    else:
        output_str = subprocess.run(f'powershell cat in/{i:04}.txt | .\\target\\debug\\ahc057.exe > out/{i:04}.txt', shell=True, capture_output=True, text=True).stderr
    # print('output_str:', output_str.split('\n'))
    result = json.loads(output_str.split('\n')[-2])
    return result


def main(i, params=None, tuning=False):
    start = time.time()
    # print(i, 'start')
    r = run(i, params, tuning)
    t = round(time.time()-start, 4)
    score = r['score']
    data = [i, score, t]
    print('\r', 'end', i, end='')
    # print(i, 'end')
    return data


def run_simulate(trial=150, params=None, tuning=False):
    start = time.time()
    if not tuning:
        print("start: ", datetime.datetime.fromtimestamp(start))
    '''
    result = []
    for i in tqdm(range(trial)):
        data = main(i, params, tuning)
        result.append(data)
    '''
    processes = multiprocessing.cpu_count()
    with multiprocessing.Pool(processes=processes) as pool:
        if tuning:
            data = [pool.apply_async(main, (i, params,)) for i in range(trial)]
        else:
            data = [pool.apply_async(main, (i,)) for i in range(trial)]
        result = [d.get() for d in data]
    print()
    # '''
    df = pd.DataFrame(result, columns=['i', 'score', 'time'])
    score = np.mean(df['score'])
    sum_score = score * 150
    if not tuning:
        print(f"score: {format(int(sum_score), ',')}, score mean: {format(int(score), ',')}")
        df.to_csv('result.csv', index=False)
        print(f'end elapsed time: {time.time()-start:.2f}s')
    return score


if __name__ == '__main__':
    run_simulate()

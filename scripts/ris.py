import matplotlib.pyplot as plt
import numpy as np
from collections import Counter



class Reservoir:
    def __init__(self, x, w) -> None:
        self.y = x # sample
        self.wSum = w # sum of the weights

    def update(self, x, w):
        '''
        x: new candidate
        w: its weight
        '''

        self.wSum += w
        if np.random.rand() < w / self.wSum:
            # update 
            self.y = x
        
        # end


def p(x):
    '''
    returns an evenly spaced sample 
    '''
    return 1

def targetPdf(x):
    if x < 0.5:
        return 4 * x
    else:
        return 4 - 4 * x

def ris_wrs(m):
    '''
    m: the number of candidates 
    '''
    r = Reservoir(np.random.rand(), p(1))

    for _ in range(m-1):
        candidiate = np.random.rand()
        w = targetPdf(candidiate) / p(candidiate)
        r.update(candidiate, w)
    
    return r.y


if __name__ == "__main__":
    # 
    rec = Counter()
    for _ in range(1000):
        y = ris(4)
        rec[y] += 1
    
    xs = sorted(list(rec.keys()))
    ys = [targetPdf(x) for x in xs]
    
    hist, bins = np.histogram(np.array(xs), bins=10, density=True)
    bin_centers = 0.5 * (bins[:-1] + bins[1:])

    plt.plot(xs, ys)
    plt.plot(bin_centers, hist)

    plt.show()
import numpy as np
import os
import argparse
import glob

LANG_MAP = {
    "af": "afr",
    "bg": "bul",
    "cs": "ces",
    "da": "dan",
    "nl": "nld",
    "de": "deu",
    "en": "eng",
    "et": "est",
    "fi": "fin",
    "fr": "fra",
    "el": "ell",
    "hu": "hun",
    "it": "ita",
    "lv": "lav",
    "lt": "lit",
    "pl": "pol",
    "pt": "por",
    "ro": "ron",
    "sk": "slk",
    "sl": "slv",
    "es": "spa",
    "sv": "swe",
}

def cosAngle(u, v):
    #    d=np.dot(u[0],v[0])
    #    n1=np.linalg.norm(u[0])
    #    n2=np.linalg.norm(v[0])
    #    return d/(n1*n2)
    return np.dot(u[0], v[0]) / (np.linalg.norm(u[0]) * np.linalg.norm(v[0]))

def init_hv(N):
    assert N % 2 == 0, "N is odd"
    a = np.random.choice([-1, 1], size=N // 2, replace=True)
    a = np.concatenate([a, -a])
    np.random.shuffle(a)
    return a

def computeSumHV(text, letters, N, D):
    block = np.zeros((N, D))
    sumHV = np.zeros((1, D))
    for i, key in enumerate(text):
        block = np.roll(block, shift=(1, 1), axis=(0, 1))
        v = letters.setdefault(key, init_hv(D))
        block[0] = v
        if i >= N:
            nGram = block[0]
            for v in block[1:]:
                nGram = np.multiply(nGram, v)  # Hadamar
            sumHV += nGram
    return sumHV

def train(N, D):
    symbols = {}
    languages = {}
    for i, lang in enumerate(LANG_MAP.values()):
        fname = "../training_texts/" + lang + ".txt"
        text = open(fname).read()
        print(f"{i+1}/{len(LANG_MAP)}: Loaded training file {fname}")
        languages[lang] = computeSumHV(text, symbols, N, D)
    return symbols, languages

def test(symbols, languages, N, D):
    total = 0
    correct = 0

    path = "../testing_texts/"
    for i, label in enumerate(LANG_MAP):
        alabel = LANG_MAP[label]
        for fname in glob.glob(os.path.join(path, f"{label}_*.txt")):
            with open(fname, "r") as f:
                text = f.read()
            v = computeSumHV(text, symbols, N, D)

            maxAngle, plabel = max(
                ((cosAngle(languages[label], v), label) for label in LANG_MAP.values())
            )
            if plabel == alabel:
                correct += 1
            # print(f"{alabel} --> {plabel}")
            total += 1
    print(f"Accuracy: {correct}/{total}={correct/total}")

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="train and evaluate HD language identification model."
    )
    parser.add_argument("-n", "--ngram", type=int, default=2)
    parser.add_argument("-d", "--dim", type=int, default=100)
    args = parser.parse_args()

    symbols, languages = train(args.ngram, args.dim)
    test(symbols, languages, args.ngram, args.dim)

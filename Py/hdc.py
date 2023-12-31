import numpy as np
import os
import argparse
import glob
import collections

LANG_MAP = {
    "af": "afr",
    "bg": "bul",
    "cs": "ces",
    "da": "dan",
    "de": "deu",
    "el": "ell",
    "en": "eng",
    "es": "spa",
    "et": "est",
    "fi": "fin",
    "fr": "fra",
    "hu": "hun",
    "it": "ita",
    "lt": "lit",
    "lv": "lav",
    "nl": "nld",
    "pl": "pol",
    "pt": "por",
    "ro": "ron",
    "sk": "slk",
    "sl": "slv",
    "sv": "swe",
}


def cosAngle(u, v):
    return np.dot(u, v) / (np.linalg.norm(u) * np.linalg.norm(v))


def init_hv(N):
    assert N % 2 == 0, "N is odd"
    a = np.random.choice([-1, 1], size=N // 2, replace=True)
    a = np.concatenate([a, -a])
    np.random.shuffle(a)
    return a


def computeSumHV(fname, letters, N, D):
    sumHV = np.zeros(D)
    for line in open(fname):
        block = np.ones((N, D))
        ngram = np.ones(D)
        for c in line[:N]:
            block = np.roll(block, shift=(1, 1), axis=(0, 1))
            block[0] = letters.setdefault(c, init_hv(D))
            ngram = np.multiply(np.roll(ngram, shift=1), block[0])  # Hadamard

        for c in line[N:]:
            ngram = np.multiply(ngram, block[N - 1])  # forget - Hadamard
            block = np.roll(block, shift=(1, 1), axis=(0, 1))
            block[0] = letters.setdefault(c, init_hv(D))
            ngram = np.multiply(np.roll(ngram, shift=1), block[0])  # Hadamard
            sumHV += ngram
    return sumHV


def train(N, D):
    symbols = {}
    languages = {}
    for i, lang in enumerate(LANG_MAP.values()):
        fname = "../training_texts/" + lang + ".txt"
        print(f"{i+1}/{len(LANG_MAP)}: Processing {fname}")
        languages[lang] = computeSumHV(fname, symbols, N, D)
    return symbols, languages


def test(symbols, languages, N, D):
    total = 0
    correct = 0

    path = "../testing_texts/"
    d = collections.defaultdict(int)
    for i, label in enumerate(LANG_MAP):
        for fname in glob.glob(os.path.join(path, f"{label}_*.txt")):
            v = computeSumHV(fname, symbols, N, D)
            maxAngle, plabel = max(
                ((cosAngle(languages[label], v), label) for label in LANG_MAP.values())
            )
            if plabel == LANG_MAP[label]:
                correct += 1
            total += 1
            d[(plabel, LANG_MAP[label])] += 1
        if total > 0:
            print(f"+{i+1} {label}: Accuracy: {correct}/{total}={correct/total}")
    display_confusions(d)


def display_confusions(d, format="plain"):
    v = sorted(LANG_MAP.values())
    if format == "plain":
        print([s for s in v])
        for s in v:
            print([d[(s, x)] for x in v])
    else:  # markdown
        print("||" + "|".join(v) + "|")
        print("|-:|" + "|".join("-:" for x in v) + "|")
        for i,s in enumerate(v):
            print(f"|{v[i]}|" + "|".join(str(d[(s, x)]) for x in v) + "|")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="train and evaluate HD language identification model."
    )
    parser.add_argument("-n", "--ngram", type=int, default=2)
    parser.add_argument("-d", "--dim", type=int, default=100)
    args = parser.parse_args()

    symbols, languages = train(args.ngram, args.dim)
    test(symbols, languages, args.ngram, args.dim)

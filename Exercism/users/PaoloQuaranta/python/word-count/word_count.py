import re

def count_words(sentence):
    lowered = sentence.lower()
    word_count = {}
    res = re.findall(r"[a-z0-9]+(?:['][a-z]+)*", lowered)
    for word in res:
        if word not in word_count:
            word_count[word] = 1
        else:
            word_count[word] += 1
    return word_count
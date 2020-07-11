import pandas as pd

df = pd.read_table(
    "http://people.bu.edu/kalathur/datasets/speeches.txt", sep="#")

for iterate_rows in df.iterrows():
    index_de_row = iterate_rows[0]
    values_de_row = dict(iterate_rows[1])
    cv_c = CountVectorizer(stop_words="english")
    cv_c.fit([values_de_row["speech"]])
    vocab_english = cv_c.vocabulary_
    vocabulary_sorted_format = {k: v for k, v in sorted(
        vocab_english.items(), key=lambda item: item[1])}
    freq_maxima = vocabulary_sorted_format[list(
        vocabulary_sorted_format.keys())[-1]]
    for i in vocabulary_sorted_format:
        if vocabulary_sorted_format[i] == freq_maxima:
            upper_format = i.upper()
            print(upper_format)

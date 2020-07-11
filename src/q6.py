import pandas as pd
from sklearn.metrics.pairwise import cosine_similarity

df = pd.read_csv("http://people.bu.edu/kalathur/datasets/item_ratings.csv",index_col=0 )

simulation_for_users = cosine_similarity(df.values,df.values)
neighbouring_users = simulation_for_users.argmin(axis =1)

item_sim = cosine_similarity(df.T.values,df.T.values)
closest_items = item_sim.argmin(axis =1)

print('Items similarity: ')
for item in range(len(closest_items)):
    print('Item {} is closest to item {}'.format(item, closest_items[item]))

print('Users similarity: ')
for user in range(len(neighbouring_users)):
    print('User {} is closest to user: {}'.format(user, neighbouring_users[user]))
# print(closest_items)
# print(neighbouring_users)
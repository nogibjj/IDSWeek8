import pandas as pd
import time
import os
import psutil

start_time = time.time()

# Read dataset
df = pd.read_csv('games_details.csv')

# Sort by 'PTS'
sorted_df = df.sort_values(by='PTS', ascending=False)

# Optionally: Save sorted dataframe to a new CSV file
# sorted_df.to_csv('sorted_games_details.csv', index=False)

end_time = time.time()

print(f"Execution Time: {end_time - start_time} seconds")
process = psutil.Process(os.getpid())
print(f"Memory Usage: {process.memory_info().rss / 1024**2} MB")

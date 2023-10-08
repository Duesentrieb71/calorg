from datetime import datetime, timedelta

# Define the UTC timestamp
utc_timestamp = "20220328T123400Z"

# Convert the UTC timestamp to a datetime object
utc_datetime = datetime.strptime(utc_timestamp, "%Y%m%dT%H%M%SZ")

# Add the time difference between UTC and Berlin (+2 hours)
berlin_datetime = utc_datetime + timedelta(hours=2)

# Convert the Berlin datetime object to a string in the desired format
berlin_timestamp = berlin_datetime.strftime("%Y%m%dT%H%M%S")

print(berlin_timestamp)

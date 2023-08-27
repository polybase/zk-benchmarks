import firebase_admin
from firebase_admin import credentials, firestore
import json
import os

# Initialize Firestore
service_account_data = os.environ.get('FIRESTORE_CREDENTIALS')  # Assuming you pass the credentials as an environment variable
cred = credentials.Certificate(json.loads(service_account_data))
firebase_admin.initialize_app(cred)
db = firestore.client()

# Directory containing the JSON files
directory_path = '.benchmarks'

benchmarks = {}

# Iterate through each file in the directory
for filename in os.listdir(directory_path):
    file_path = os.path.join(directory_path, filename)
    file_without_extension = os.path.splitext(filename)[0]
    
    # Ensure the file is a JSON before processing (optional but recommended)
    if file_path.endswith('.json'):
        with open(file_path, 'r') as f:
            benchmarks[file_without_extension] = json.load(f)


# Save to Firestore
doc_ref = db.collection('benchmarks').document()  # Creates a new document in 'benchmarks' collection
doc_ref.set(benchmarks)
print(f'Successfully uploaded to Firestore')
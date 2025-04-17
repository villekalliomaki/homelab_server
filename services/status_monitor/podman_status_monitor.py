# Monitor podman container states, and notify on changes
import json
import logging
import subprocess
import os
import requests

# Paths
container_states_path = os.path.expanduser("~/podman_status_monitor.json")
container_users_path = os.path.expanduser("~/scripts/podman_users.txt")
ntfy_channel_path = os.path.expanduser("~/secrets/ntfy_channel.txt")

# Log config
logging.basicConfig(level=logging.INFO)

# Read ntfy channel ID
with open(ntfy_channel_path, "r") as file:
    ntfy_channel = file.read().replace("\n", "")
    ntfy_url = f"https://ntfy.sh/{ntfy_channel}"

# Try to get previous states of pods
#
# Format:
# - Username
#   - Pod name: pod state
container_states = {}

try:
    # Use the saved state
    with open(container_states_path, "r") as file:
        container_states = json.load(file)

    with open(container_users_path, "r") as file:
        for username in file:
            # Strip newline
            username = username.rstrip()

            logging.info(f"Running for user: {username}")

            # Get past states of current user's containers
            user_container_states = container_states.get(username, {})
        
            # Define command
            podman_command = 'podman ps -a --format "{{.ID}};{{.Names}};{{.State}}"'

            command = [
                "su",
                "-",
                username,
                "-c",
                podman_command
            ]

            # Run the command as a specified user
            ps_command_result = subprocess.run(command, stdout=subprocess.PIPE).stdout.decode('utf-8')

            # For each container
            for line in ps_command_result.splitlines():
                if len(line) <= 1:
                    # Skip empty lines
                    continue

                # Parse command output
                result_split = line.split(";")

                ps_result = {
                    "id": result_split[0],
                    "name": result_split[1],
                    "state": result_split[2]
                }

                # Compare to previous state, if it exists
                container_state = user_container_states.get(ps_result["name"], None)

                # If not running anymore
                if ps_result["state"] != "running" and container_state == "running":
                    headers = {
                        "Title": "Container state alert",
                        "Priority": "high",
                        "Tags": "warning"
                    }

                    logging.info(f"Sending alert for {ps_result["name"]}, state change to {ps_result["state"]}")

                    # Last state was running, and now somehting else = notify
                    requests.post(ntfy_url, f"{ps_result["name"]} state changed to {ps_result["state"]} ({username}, {ps_result["id"]})", headers=headers)

                # If running after other state
                if ps_result["state"] == "running" and container_state != "running":
                    headers = {
                        "Title": "Container state alert",
                        "Tags": "white_check_mark"
                    }

                    logging.info(f"Sending alert for {ps_result["name"]}, state change to {ps_result["state"]}")

                    requests.post(ntfy_url, f"{ps_result["name"]} state changed to {ps_result["state"]} ({username}, {ps_result["id"]})", headers=headers)

                # Save new state
                user_container_states[ps_result["name"]] = ps_result["state"]

            # Then save user's container states
            container_states[username] = user_container_states

    with open(container_states_path, "w") as file:
        # Write state to file
        json.dump(container_states, file)

# Catch any errors
except Exception as error:
    logging.warning(f"Error occurred: {error}")

    headers = {
        "Title": "Error with container monitor",
        "Priority": "urgent",
        "Tags": "bangbang"
    }

    requests.post(ntfy_url, f"Error while running status monitor: {error}", headers=headers)
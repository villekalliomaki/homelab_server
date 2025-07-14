#!/bin/bash

# Get current forwarded port
gluetun_res=$(curl \
                    -s \
                    -H "X-API-Key: {{ gluetun.api_key }}" \
                    http://{{ network.internal_ip }}:{{ network.service_ports.qbittorrent_gluetun }}/v1/openvpn/portforwarded \
                )

# Parse reponse JSON
gluetun_port=$(echo "$gluetun_res" | jq -r ".port")

echo "Updating forwarded port to $gluetun_port"

# Create a session
qbittorrent_login_res=$(curl \
                            -s \
                            -o /dev/null \
                            -w "%{http_code}" \
                            -c "{{ directories_volume_subdirectories_output.results[4].path }}/qbittorrent_cookies.txt" \
                            -H "Referer: http://{{ network.internal_ip }}:{{ network.service_ports.qbittorrent_web_ui }}" \
                            --data "username={{ qbittorrent.username }}&password={{ qbittorrent.password }}" \
                            http://{{ network.internal_ip }}:{{ network.service_ports.qbittorrent_web_ui }}/api/v2/auth/login \
                        )

echo "Created a Qbittorrent session with cookie: $qbittorrent_login_res"

# Update port
qbittorrent_res=$(curl \
                        -s \
                        -b "{{ directories_volume_subdirectories_output.results[4].path }}/qbittorrent_cookies.txt" \
                        -o /dev/null \
                        -w "%{http_code}" \
                        --data 'json={"listen_port": '"$gluetun_port"'}' \
                        http://{{ network.internal_ip }}:{{ network.service_ports.qbittorrent_web_ui }}/api/v2/app/setPreferences \
                    )

echo "Port updated HTTP response: $qbittorrent_res"

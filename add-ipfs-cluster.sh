#!/usr/bin/env bash
REMOTE_HOST=${1-"18.216.57.14"}
DEFAULT_API_FILE="$HOME/.ipfs/api"
API_FILE="${IPFS_PATH-$DEFAULT_API_FILE}"

if [ -e "$API_FILE" ]; then
	echo "IPFS API is already running"
	exit 1
fi

PORT=5001
ssh -N -L ${PORT}:localhost:5001 $REMOTE_HOST &
SSH_PID=$!

function cleanupAndExit() {
	rm -f "$API_FILE"
	echo
	echo "Killing ssh."
	kill "$SSH_PID"
	exit
}
trap cleanupAndExit INT

printf "/ip4/127.0.0.1/tcp/$PORT" > "$API_FILE"

echo "Linked local API to $REMOTE_HOST."

wait $SSH_PID
rm -f "$API_FILE"

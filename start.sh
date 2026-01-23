#!/bin/bash

# Timelapse Creator - Docker Startup Script
# This script starts the application and shows where to access it

# Default port is 80, can be overridden with PORT environment variable
PORT="${PORT:-80}"

echo "Starting Timelapse Creator..."
echo ""

# Check if port is in use
if lsof -Pi :$PORT -sTCP:LISTEN -t >/dev/null 2>&1; then
    echo "WARNING: Port $PORT is already in use!"
    echo ""
    echo "Options:"
    echo "  1. Stop the application using port $PORT"
    echo "  2. Use a different port: PORT=8000 ./start.sh"
    echo ""
    read -p "Do you want to try a different port? (y/n): " choice
    if [[ "$choice" == "y" || "$choice" == "Y" ]]; then
        read -p "Enter port number: " PORT
        export PORT
    else
        echo "Exiting..."
        exit 1
    fi
fi

# Export PORT for docker-compose
export PORT

# Start the containers
docker-compose up -d --build

# Check if startup was successful
if [ $? -eq 0 ]; then
    echo ""
    echo "=========================================="
    echo "  Timelapse Creator is running!"
    echo "=========================================="
    echo ""
    if [ "$PORT" == "80" ]; then
        echo "  Frontend URL: http://localhost"
    else
        echo "  Frontend URL: http://localhost:$PORT"
    fi
    echo ""
    echo "  Useful commands:"
    echo "    View logs:    docker-compose logs -f"
    echo "    Stop:         docker-compose down"
    echo "    Status:       docker-compose ps"
    echo "=========================================="
else
    echo ""
    echo "ERROR: Failed to start containers. Check the logs with:"
    echo "  docker-compose logs"
    exit 1
fi

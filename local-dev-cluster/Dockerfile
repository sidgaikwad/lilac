# Use the official Jupyter base notebook image as a parent image
FROM jupyter/base-notebook:latest

# Switch to root user to install packages
USER root

# Install vim and other utilities
RUN apt-get update && apt-get install -y --no-install-recommends \
    vim \
    && rm -rf /var/lib/apt/lists/*

# Switch back to the jovyan user for Python package installations
USER jovyan

# Install Python ML libraries
RUN pip install --no-cache-dir \
    scikit-learn \
    pandas \
    matplotlib \
    seaborn \
    tensorflow \
    torch

# Switch back to root to modify the Jupyter config
USER root

# Append the CSP header configuration to allow iframe embedding
RUN echo "c.ServerApp.tornado_settings = {" >> /etc/jupyter/jupyter_server_config.py && \
    echo "    'headers': {" >> /etc/jupyter/jupyter_server_config.py && \
    echo "        'Content-Security-Policy': 'frame-ancestors *;'" >> /etc/jupyter/jupyter_server_config.py && \
    echo "    }" >> /etc/jupyter/jupyter_server_config.py && \
    echo "}" >> /etc/jupyter/jupyter_server_config.py

# Switch back to the jovyan user to run the notebook server
USER jovyan
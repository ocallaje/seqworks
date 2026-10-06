# Dockerfile for serving the bootstrap frontend
# Use the official nginx image as a lightweight static file server
FROM nginx:alpine

# Copy the entire frontend source (src directory) into the nginx html dir
# The target directory in the nginx image is /usr/share/nginx/html
COPY src/ /usr/share/nginx/html/

# Expose port 80 so the container can be accessed from the host
EXPOSE 80

# Default nginx command will start the server in the foreground
CMD ["nginx", "-g", "daemon off;"]

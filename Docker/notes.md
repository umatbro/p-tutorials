# Useful commands

* ``docker images`` - lists all downloaded images
* ``docker container rm $(docker ps -a -q)`` - will delete all containers


# building container images

The ``CMD`` line in a Dockerfile defines the default command to run when a container is launched. If the command requires arguments then you need to use an array, for example ``["cmd", "-a", "arga value", "-b", "argb-value"]``, which will be combined together and the command ``cmd -a arga value -b argb-value`` would be run.


To build a container from a ``Dockerfile`` run command ``docker build -t my-name:v1 .``.

Argument in ``-t`` flag is a custom image name, the last argument is a ``Dockerfile`` location.

## Launching new image

``docker run [OPTIONS] <image_name>``

* ``-e`` flag sets container environmental variables, eg. ``docker run -e NODE_ENV=production -p 3000:3000 my-nodejs-app``

# ignoring files

To prevent sensitive files or directories from being included by mistake in images, you can add a file named .dockerignore.

# data containers

To create a Data Container we first create a container with a well-known name for future reference. We use busybox as the base as it's small and lightweight in case we want to explore and move the container to another host.

When creating the container, we also provide a -v option to define where other containers will be reading/saving data.

``docker create -v /config --name dataContainer busybox``

With the container in place, we can now copy files from our local client directory into the container.

To copy files into a container you use the command docker cp. The following command will copy the config.conf file into our dataContainer and the directory config.

``docker cp config.conf dataContainer:/config/``



If we wanted to move the Data Container to another machine then we can export it to a .tar file.

``docker export dataContainer > dataContainer.tar``

The command ``docker import dataContainer.tar`` will import the Data Container back into Docker.

# Communicating between containers

The most common scenario for connecting to containers is an application connecting to a data-store. The key aspect when creating a link is the name of the container. All containers have names, but to make it easier when working with links, it's important to define a friendly name of the source container which you're connecting to.

# Start Data Store

Run a redis server with a friendly name of redis-server which we'll connect to in the next step. This will be our source container.

``docker run -d --name redis-server redis``



# Other tutorials

* https://training.play-with-docker.com/beginner-linux/
* example voting app: https://github.com/dockersamples/example-voting-app
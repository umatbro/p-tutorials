# Useful commands

* ``docker images`` - lists all downloaded images
* ``docker container rm $(docker ps -a -q)`` - will delete all containers


# building container images

The ``CMD`` line in a Dockerfile defines the default command to run when a container is launched. If the command requires arguments then you need to use an array, for example ``["cmd", "-a", "arga value", "-b", "argb-value"]``, which will be combined together and the command ``cmd -a arga value -b argb-value`` would be run.


To build a container from a ``Dockerfile`` run command ``docker build -t my-name:v1 .``.

Argument in ``-t`` flag is a custom image name, the last argument is a ``Dockerfile`` location.

## Launching new image

``docker run [OPTIONS] <image_name>``

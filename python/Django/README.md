# Django tutorial
Follownig book ***How to Tango with Django***

## Notes
### Sharing package list
It is possible to get list of packages installed by executing following command:
```bash
$ pip freeze > requirements.txt
```

On another PC same dependancies can be installed by running command
```bash
$ pip install -r requirements.txt
```

### Creating new Django project
Run command
```
$ django-admin startproject <project_name>
```

### Run dev server
To start lightweight development server run command
```
$ python manage.py runserver <ip_adress:port>
```

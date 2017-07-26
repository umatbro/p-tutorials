# Django tutorial
Follownig book ***How to Tango with Django*** and ***[Django official tutorial](https://docs.djangoproject.com/en/1.11/intro/tutorial01/)***

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

### Creating new Django application
To create new application run command
```
$ python manage.py startapp <app_name>
```
Remember to put application name in `INSTALLED_APPS` list in `settings.py` file in your project directory.
```python
# settings.py
# (...)
INSTALLED_APPS = [
    'django.contrib.admin',
    # (...)
    'rango', # new app name
    'polls.apps.PollsConfig', # attaching config method from apps package (probably better practice)
]
```

Also remember in project's `urls.py` add mapping to the application

```python
# urls.py
from django.conf.urls import include, url
from django.contrib import admin

urlpatterns = [
    url(r'^admin/', admin.site.urls),
    url(r'^rango/', include('rango.urls')),
]
```
You can also create new file `urls.py` in app directory to map URLs to views. Those views should be configured in `views.py` file.

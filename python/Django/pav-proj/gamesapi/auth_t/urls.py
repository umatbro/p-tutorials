from django.urls import path
from auth_t import views
from auth_t.apps import AuthTConfig

app_name = AuthTConfig.name

urlpatterns = [
    path('', views.home, name='home'),
    path('login/', views.login, name='login'),
    path('logout/', views.logout, name='logout'),
    path('register/', views.register, name='register'),
    path('secret/', views.secret, name='secret'),
]

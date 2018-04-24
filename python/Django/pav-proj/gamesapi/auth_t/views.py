from django.http import HttpResponse
from django.contrib import auth
from django.shortcuts import render, redirect
from django.contrib.auth.forms import UserCreationForm, AuthenticationForm


def home(request):
    return render(request, 'auth_t/home.html', {'user': request.user})


def secret(request):
    pass


def login(request):
    return HttpResponse('Not implemented yet')
    if request.method == 'POST':
        form = AuthenticationForm(request.POST)

    return render(request, 'auth_t/login.html', {
        'form': AuthenticationForm()
    })


def logout(request):
    auth.logout(request)
    return redirect('auth_t:home')


def register(request):
    if request.method == 'POST':
        form = UserCreationForm(request.POST)

        if form.is_valid():
            new_user = form.save()
            auth.login(request, new_user)
            return redirect('auth_t:home')
        else:
            msg = 'Invalid input' + str(form.errors)
            return HttpResponse(msg)

    form = UserCreationForm()
    return render(request, 'auth_t/register.html', {'form': form})

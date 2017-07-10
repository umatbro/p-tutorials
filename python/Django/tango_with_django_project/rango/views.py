# -*- coding: utf-8 -*-
from __future__ import unicode_literals

from django.shortcuts import render

# Create your views here.
from django.http import HttpResponse

def index(request):
    return render(request, 'rango/index.html', {
        'boldmessage' : 'I\'m bold font from the context'
    })

def about(request):
    return HttpResponse("Here is the about page")

def chapter4(request):
    data = {
        "Kas" : "ass",
        "Tal" : "ass",
        "Ash" : "mrk",
        "TwF" : "mag",
        "Jce" : "war",
        "War" : "war",
        "Zac" : "tnk",
    }
    return render(request, 'rango/chapter4.html', { 'data' : data })

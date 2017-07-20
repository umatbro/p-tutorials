# -*- coding: utf-8 -*-
from __future__ import unicode_literals

from django.shortcuts import render

from django.http import HttpResponse

# Create your views here.
def index(request):
    return HttpResponse("Poll index")

def detail(request, question_id):
    return HttpResponse("You're looking at question %s" % question_id)

def results(request, question_id):
    response = "You're looking at the result of question %s"
    return HttpResponse(response % question_id)

def vote (request, question_id):
    return HttpResponse("You're voting on question %s" % question_id)

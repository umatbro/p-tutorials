import graphene
from graphene_django import DjangoObjectType
from links.models import Link, Vote
from users.schema import UserType
from graphql import GraphQLError
from django.db.models import Q


class LinkType(DjangoObjectType):
  class Meta:
    model = Link


class VoteType(DjangoObjectType):
  class Meta:
    model = Vote


class Query(graphene.ObjectType):
  links = graphene.List(
    LinkType,
    search=graphene.String(),
    first=graphene.Int(),
    skip=graphene.Int(),
  )
  votes = graphene.List(VoteType)

  def resolve_links(self, info, search=None, first=None, skip=None, **kwargs):
    qs = Link.objects.all()
    if search:
      query = (
        Q(url__icontains=search) | 
        Q(description__icontains=search)
      )
      qs = qs.filter(query)
    
    if skip:
      qs = qs[skip:]

    if first:
      qs = qs[:first]

    return qs

  def resolve_votes(self, info, **kwargs):
    return Vote.objects.all()


class CreateLink(graphene.Mutation):
  id = graphene.Int()
  url = graphene.String()
  description = graphene.String()
  posted_by = graphene.Field(UserType)

  class Arguments:
    url = graphene.String()
    description = graphene.String()

  def mutate(self, info, url, description):
    user = info.context.user or None

    link = Link(
      url=url,
      description=description,
      posted_by=user,
    )
    link.save()

    return CreateLink(
      id=link.id,
      url=link.url,
      description=link.description,
      posted_by=link.posted_by,
    )


class CreateVote(graphene.Mutation):
  link = graphene.Field(LinkType)
  user = graphene.Field(UserType)

  class Arguments:
    link_id = graphene.Int()
  
  def mutate(self, info, link_id):
    user = info.context.user
    if user.is_anonymous:
      raise GraphQLError('You must be logged in to vote.')

    link = Link.objects.filter(id=link_id).first()
    if not link:
      raise Exception('Invalid link id')

    Vote.objects.create(link=link, user=user)

    return CreateVote(
      user=user,
      link=link,
    )

class Mutation(graphene.ObjectType):
  create_link = CreateLink.Field()
  create_vote = CreateVote.Field()

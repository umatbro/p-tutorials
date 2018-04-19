from datetime import datetime
from django.utils import timezone
from django.utils.six import BytesIO
from rest_framework.renderers import JSONRenderer
from rest_framework.parsers import JSONParser
from games.models import Game
from games.serializers import GameSerializer


gamedatetime = timezone.make_aware(datetime.now(), timezone.get_current_timezone())
game1 = Game(name='Smurfs', release_date=gamedatetime, game_category='2D mobile', played=False)
game1.save()
game2 = Game(name='Angry Birds RPG', release_date=gamedatetime, game_category='3D RPG', played=False)
game2.save()


game_serializer = GameSerializer(game1)

renderer = JSONRenderer()
rendered_g1 = renderer.render(game_serializer.data)

json_string_for_new_game = '{"name":"Tomb Raider ExtremeEdition","release_date":"2016-05-18T03:02:00.776594Z","game_category":"3D RPG","played":false}'
json_bytes = bytes(json_string_for_new_game, encoding='utf-8')
stream = BytesIO(json_bytes)
parser = JSONParser()
parsed_new_game = parser.parse(stream)

new_game_serializer = GameSerializer(data=parsed_new_game)
if new_game_serializer.is_valid():
    new_game = new_game_serializer.save()

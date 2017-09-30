"""Module to easily maintain charts using Chart.js library"""
import json
from collections import namedtuple
from copy import deepcopy
from motool.magic.utils import merge_dicts
from colors import Color
from options import ChartType, LINE_CHART_DATASET_OPTIONS


class Chart:
    def __init__(self, datasets=None, title=None, options=None, id='chart', chart_type=ChartType.BAR):
        if options is None:
            options = {}
        self.title = title
        self.id = id  # id will be used to set HTML canvas id and JavaScript variable name
        self.chart_type = chart_type
        if isinstance(datasets, Dataset):
            self.datasets = [datasets]
        elif isinstance(datasets, list):
            self.datasets = datasets
        else:
            raise TypeError('Wrong dataset type')

        self.options = options

    def get_data_dict(self, decimal_places=6, sort=True):
        """
        :return dictionary representing data object
        will force all datasets to have same arguments
        """
        # create labels set
        labels = set()
        datasets_copy = deepcopy(self.datasets)
        for dataset in datasets_copy:
            for argument in dataset.data.keys():
                labels.add(argument)
                if argument not in dataset.data.keys():
                    dataset.data[argument] = None

        # sort labels depending on the 'sort' flag
        sorted_labels = sorted(labels) if sort else labels

        datasets = [merge_dicts({
            'label': dataset.label,
            'data': [round(dataset.data[label], decimal_places) for label in sorted_labels]
        }, dataset.settings) for dataset in datasets_copy]
        return {
            'labels': [label for label in sorted_labels],
            'datasets': datasets
        }

    def get_dict(self):
        """
        :return dict that can be transformed to JSON.
        This data can be used to create JavaScript Chart object from Chart.js library
        """
        return {
            'type': self.chart_type,
            'data': self.get_data_dict(),
            'options': self.options
        }

    @property
    def json(self):
        return json.dumps(self.get_dict())

    def create_js_var(self):
        return 'var {0} = new Chart(document.getElementById("{0}").getContext("2d"), JSON.parse(\'{1}\'))'.format(
            self.id,
            self.json
        )

    def set_options(self, options):
        self.options = options

    @property
    def html(self):
        return '<canvas id="{}"></canvas>'.format(self.id)

    @property
    def js(self):
        return self.create_js_var()

    @property
    def code(self):
        return '{}\n<script>\n\t{}\n</script>'.format(self.html, self.js)

    def get_code(self, title=None):
        """
        :return named tuple containing html and js
        """
        if title is None:
            title = self.title
        ChartTuple = namedtuple('Chart', ['title', 'html', 'js'])
        return ChartTuple(title=title, html=self.html, js=self.js)


class Dataset:
    def __init__(self, label='', arguments=None, values=None, settings=None):
        """
        :param label: Dataset label (name of data series)
        :param arguments: x axis
        :param values: y axis
        :param settings: additional settings. Must be a JS-like object dictionary.
        """
        if values is None:
            values = []
        if arguments is None:
            arguments = []
        if settings is None:
            settings = {}

        self.label = label
        self.data = dict(zip(arguments, values))
        self.settings = settings

    def get_dict(self):
        """
        :return: Python dictionary that can be easily converted to JSON (js function: JSON.parse())
        """
        return merge_dicts({
            'label': self.label,
            'data': [self.data[argument] for argument in self.data.keys()]
        }, self.settings)

    @property
    def json(self):
        return json.dumps(self.get_dict())

    def __str__(self):
        return self.json


def dataset_from_json(json_data, label='', border_color=Color.DEFAULT_COLOR, settings=None):
    """
    Create Dataset object out of JSON.
    JSON should have two fields with arrays: arguments and values.

    :param border_color: color of the dataset on the chart
    :param label: name of dataset
    :param settings: additional settings (passed as dictionary)
    :param json_data: JSON string
    :return: Dataset object
    """
    if settings is None or not isinstance(settings, dict):
        settings = {}
    json_data = json.loads(json_data)
    try:
        return Dataset(
            label=label,
            arguments=json_data['arguments'],
            values=json_data['values'],
            settings=merge_dicts(LINE_CHART_DATASET_OPTIONS, {'borderColor': border_color}, settings)
        )
    except KeyError:
        return None

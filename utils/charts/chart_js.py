"""
Module to easily maintain charts using Chart.js library
"""

import json
from collections import namedtuple
from copy import deepcopy
from motool.magic.utils import merge_dicts
from colors import Color
from options import ChartType, LINE_CHART_DATASET_OPTIONS


class Chart(object):
    """
    Class to hold information about chart contents

    One chart can store multiple datasets (which means you will be able to display many lines on one line chart)
    """
    def __init__(self, datasets=None, title=None, options=None, chart_id='chart', chart_type=ChartType.BAR):
        if options is None:
            options = {}
        if isinstance(datasets, Dataset):
            self.datasets = [datasets]
        elif datasets is None:
            self.datasets = []
        elif isinstance(datasets, list):
            self.datasets = datasets
        else:
            raise TypeError('Dataset not valid')

        self.title = title
        self.id = chart_id  # id will be used to set HTML canvas id and JavaScript variable name
        self.chart_type = chart_type
        self.options = options

    def __len__(self):
        return len(self.datasets)

    def __nonzero__(self):
        """:return: False if all datasets are empty"""
        return any(bool(dataset) for dataset in self.datasets)

    def get_data_dict(self, decimal_places=6, sort=True):
        """
        :param decimal_places: number of decimal places to be displayed on chart
        :param sort: if this flag is set to True labels on x-axis will be sorted
        :return: dictionary representing data object. All datasets will have same set of arguments
        """
        # create labels set
        labels = set()
        datasets_copy = deepcopy(self.datasets)
        for dataset in datasets_copy:
            for argument in dataset.data.keys():
                labels.add(argument)

        for dataset in datasets_copy:
            for label in labels:
                if label not in dataset.data.keys():
                    dataset.data[label] = None

        # sort labels depending on the 'sort' flag
        sorted_labels = sorted(labels) if sort else labels

        # round floats
        def round_data(dataset, ndigits):
            result = []
            for label in sorted_labels:
                value = dataset.data[label]
                if isinstance(value, float):
                    value = round(value, ndigits)
                result.append(value)

            return result

        datasets = [merge_dicts({
            'label': dataset.label,
            'data': round_data(dataset, decimal_places)
        }, dataset.settings) for dataset in datasets_copy]

        return {
            'labels': [label for label in sorted_labels],
            'datasets': datasets
        }

    def get_dict(self):
        """
        :return: dict that can be transformed to JSON.
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

    @property
    def html(self):
        """:return: HTML to generate chart"""
        return '<canvas id="{}"></canvas>'.format(self.id)

    @property
    def js(self):
        """:return: JavaScript code required to display chart"""
        return self.create_js_var()

    @property
    def code(self):
        """
        Just call this in your template and chart will appear
        Not the best practice though.

        **Better solution**:
        In your template use chart.html in the place you want to use it and chart.js just before the end of the body section.
        """
        return '{}\n<script>{}</script>'.format(self.html, self.js)

    def create_js_var(self):
        """
        :return: string with valid JavaScript code
        """
        return 'var {0} = new Chart(document.getElementById("{0}").getContext("2d"), JSON.parse(\'{1}\'))'.format(
            self.id,
            self.json
        )

    def add_options(self, options):
        """
        Add new options to a chart (method merges current options dictionary with the new one)
        To see what options you can add see Chart.js reference
        :param options: to be added
        """
        self.options = merge_dicts(self.options, options)

    def get_code(self, title=None):
        """
        deprecated. Use html, js and code properties instead
        :return named tuple containing html and js
        """
        if title is None:
            title = self.title
        ChartTuple = namedtuple('Chart', ['title', 'html', 'js', 'object'])
        return ChartTuple(title=title, html=self.html, js=self.js, object=self)


class Dataset(object):
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

    def __len__(self):
        return len(self.data)

    @property
    def json(self):
        return json.dumps(self.get_dict())

    def get_dict(self):
        """
        :return: Python dictionary that can be easily converted to JSON (js function: JSON.parse())
        """
        return merge_dicts({
            'label': self.label,
            'data': [self.data[argument] for argument in self.data.keys()]
        }, self.settings)

    def __str__(self):
        return self.json


def dataset_from_json(json_data, label='', border_color=Color.DEFAULT_COLOR, settings=None):
    """
    Create Dataset object out of JSON.
    JSON should have two fields with arrays: arguments and values.

    :param border_color: color of the dataset on the chart
    :param label: name of dataset (function does not add label automatically so you have to do it yourself)
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

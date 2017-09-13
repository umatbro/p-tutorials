"""Module to easily maintain charts displayed on website"""
import json
from collections import namedtuple
from colors import DEFAULT_COLOR

# chart types
LINE = 'line'  # line chart
BAR = 'bar'  # bar chart

# DEFAULT VALUES

# options
DEFAULT_OPTIONS = {
    'scales': {
        'xAxes': [{
            'gridLines': {
                'display': False
            }
        }],
        'yAxes': [{
            'ticks': {
                'min': 0,
                'max': 100,
                'stepSize': 20
            }
        }]
    },
    'maintainAspectRatio': True,
    'responsive': True,
}


class Chart:
    def __init__(self, datasets=None, title=None, options=None, canvas_id='chart', var_name='chart', chart_type=BAR):
        if options is None:
            options = {}
        self.title = title
        self.canvas_id = canvas_id
        self.js_var_name = var_name
        self.chart_type = chart_type
        if isinstance(datasets, Dataset):
            self.datasets = [datasets]
        elif isinstance(datasets, list):
            self.datasets = datasets
        else:
            raise TypeError('Wrong dataset type')

        self.normalize_datasets()
        self.options = options

    def normalize_datasets(self):
        """
        Force all datasets.data to  have same keys
        If some argument has no value assigned - None will be found under this key
        :return: namedtuple ('labels', 'datasets') where labels are keys in all datasets
        """
        # get key list
        labels = set()
        for dataset in self.datasets:
            for argument in dataset.data.keys():
                labels.add(argument)

        # change datasets so all dataset.data have same keys
        for dataset in self.datasets:
            for label in labels:
                try:
                    dataset.data[label] = dataset.data[label]
                except KeyError:
                    dataset.data[label] = None
        Datasets = namedtuple('Datasets', ['labels', 'datasets'])
        return Datasets(labels=labels, datasets=self.datasets)

    def datasets_array(self):
        """:return array of dictionaries that contain dataset data"""
        return [{
            'label': dataset.label,
            'backgroundColor': dataset.background_color,
            'data': [value for argument, value in dataset.data.items()]
        } for dataset in self.datasets]

    def get_data_dict(self):
        """:return dictionary representing data object
        will force all datasets to have same arguments"""
        normalized = self.normalize_datasets()
        return {
            'labels': list(normalized.labels),
            'datasets': self.datasets_array(),
        }

    def get_dict(self):
        """:return dict that can be transformed to JSON.
         This data can be used to create JavaScript Chart object from Chart.js library"""
        return {
            'type': self.chart_type,
            'data': self.get_data_dict(),
            'options': self.options
        }

    def get_json(self):
        return json.dumps(self.get_dict())

    def create_js_var(self):
        return 'var {} = new Chart(document.getElementById("{}").getContext("2d"), JSON.parse(\'{}\'))'.format(
            self.js_var_name,
            self.canvas_id,
            self.get_json()
        )

    def set_options(self, options):
        self.options = options


class Dataset:
    def __init__(self, label='', arguments=None, values=None, background_color=DEFAULT_COLOR):
        if values is None:
            values = []
        if arguments is None:
            arguments = []

        self.label = label
        self.data = dict(zip(arguments, values))
        self.background_color = background_color

    def get_dict(self):
        """
        :return: Python dictionary that can be easily converted to JSON (js function: JSON.parse())
        """
        return {
            'label': self.label,
            'backgroundColor': self.background_color,
            'data': [values for arguments, values in self.data.items()]
        }

    def get_json(self):
        return json.dumps(self.get_dict())

    def __str__(self):
        return self.get_json()


def generate_html(chart):
    return '<canvas id="{}"></canvas>'.format(chart.canvas_id)


def generate_js(chart):
    return chart.create_js_var()


def generate_website_code(chart):
    return '{}\n<script>\n\t{}\n</script>'.format(generate_html(chart), generate_js(chart))


def datasets_from_stat_dict(stat_dict):
    """
    :param stat_dict: JSON string from rop_statistic table (column stat_dict)
    :return: list containing Dataset objects
    """
    stat_dict = json.loads(stat_dict)
    # pop labels
    labels = stat_dict.pop('labels', [])
    return [Dataset(
        label=key,
        arguments=labels,
        values=value
    ) for key, value in stat_dict.items()]


if __name__ == '__main__':
    d1 = Dataset('jeden', [1, 2, 3, 4, 5], [5, 4, 2, 3, 1])
    d2 = Dataset('dwa', [1, 3, 4, 5, ], [1, 2, 4, 5])
    d1.background_color = 'rgba(120, 1, 1, 0.5)'
    # d2.background_color = 'rgba(130, 130, 1, 0.5)'
    print(str(d1))
    chart = Chart([d1, d2], options=DEFAULT_OPTIONS, chart_type=LINE)
    print(json.dumps(chart.get_data_dict()))
    print(chart.create_js_var())
    print(generate_website_code(chart))

    for dataset in datasets_from_stat_dict('{\"Node\": [], \"Pint_R_D\": [], \"labels\": [], \"Pint_A\": [], \"AV\": [], \"time\": []}'):
        print(dataset)

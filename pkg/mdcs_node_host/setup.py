#!/usr/bin/env python

from setuptools import setup, find_packages

setup(
    name='mdcs_node_host',
    version='0.1',
    description='MDCS Node Host Plugin',
    author='Alexandru Barbur',
    author_email='alex@ctrlc.name',
    url='https://github.com/CtrlC-Root/mdcs/tree/master/pkg/mdcs_node_host',

    packages=find_packages(),
    include_package_data=True,
    install_requires=[
        'avro-python3 >= 1.8.0',
        'psutil >= 5.2.0',
        'python-daemon >= 2.0.0'
    ],

    entry_points={
        'console_scripts': [
            'mdcs-node-host=mdcs_node_host.console:main'
        ]
    }
)

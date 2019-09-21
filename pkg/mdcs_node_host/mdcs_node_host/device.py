import json
import uuid
from enum import IntFlag

import avro.schema


class AttributeFlags(IntFlag):
    """
    Attribute flags that represent capabilities and allowed usage.
    """

    READ = 1        # clients can read value
    WRITE = 2       # clients can write value


class Attribute:
    """
    An attribute that represents part of a device's current state.
    """

    def __init__(self, path, flags, schema):
        self._path = path
        self._flags = flags
        self._schema = avro.schema.Parse(json.dumps(schema))

    @property
    def path(self):
        return self._path

    @property
    def flags(self):
        return self._flags

    @property
    def schema(self):
        return self._schema

    @property
    def readable(self):
        return AttributeFlags.READ in self._flags

    @property
    def writable(self):
        return AttributeFlags.WRITE in self._flags

    def read(self):
        """
        Read the attribute value.
        """

        raise NotImplemented()

    def write(self, value):
        """
        Write the attribute value.
        """

        raise NotImplemented()


class StoredAttribute(Attribute):
    """
    An attribute that stores its value in memory.
    """

    def __init__(self, path, flags, schema, value):
        super().__init__(path, flags, schema)
        self.value = value

    def read(self):
        return self.value

    def write(self, value):
        self.value = value


class DelegatedAttribute(Attribute):
    """
    An attribute that uses external functions to read and write a value.
    """

    def __init__(self, path, flags, schema, read_handler, write_handler):
        super().__init__(path, flags, schema)
        self._read_handler = read_handler
        self._write_handler = write_handler

    def read(self):
        return self._read_handler()

    def write(self, value):
        return self._write_handler(value)


class Action:
    """
    An action that changes a device's current state.
    """

    def __init__(self, path, input_schema, output_schema):
        self._path = path
        self._input_schema = avro.schema.Parse(json.dumps(input_schema))
        self._output_schema = avro.schema.Parse(json.dumps(output_schema))

    @property
    def path(self):
        return self._path

    @property
    def input_schema(self):
        return self._input_schema

    @property
    def output_schema(self):
        return self._output_schema

    def run(self, input_data):
        """
        Run the action with the given arguments.
        """

        raise NotImplemented()


class DelegatedAction(Action):
    """
    An action that uses an external function to modify the device's current state.
    """

    def __init__(self, path, input_schema, output_schema, handler):
        super().__init__(path, input_schema, output_schema)
        self._handler = handler

    def run(self, input_data):
        return self._handler()


class Device:
    """
    An abstract device composed of configuration settings, current state, and
    actions that can modify that state.
    """

    def __init__(self, name=None, config={}):
        """
        Create a new device.
        """

        # store the device settings
        self._name = name or str(uuid.uuid4())
        self._config = config

        # device state and actions
        self._attributes = {}
        self._actions = {}

    @property
    def name(self):
        return self._name

    @property
    def config(self):
        return self._config

    @property
    def attributes(self):
        return self._attributes

    def add_attribute(self, attribute):
        if attribute.path in self._attributes or attribute.path in self._actions:
            raise KeyError("attribute path is not unique")

        self._attributes[attribute.path] = attribute

    def remove_attribute(self, attribute):
        if attribute.path not in self._attributes:
            raise KeyError("attribute not found")

        del self._attributes[attribute.path]

    @property
    def actions(self):
        return self._actions

    def add_action(self, action):
        if action.path in self._attributes or action.path in self._actions:
            raise KeyError("action path is not unique")

        self._actions[action.path] = action

    def remove_action(self, action):
        if action.path not in self._actions:
            raise KeyError("action not found")

        self._actions[action.path] = action

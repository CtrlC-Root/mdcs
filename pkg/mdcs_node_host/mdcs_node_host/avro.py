import time
import json
import socket
from io import BytesIO

import pkg_resources
import avro.protocol
from avro.schema import AvroException
from avro.io import DatumReader, DatumWriter
from avro.datafile import DataFileReader, DataFileWriter
from avro.ipc import Responder, FramedReader, FramedWriter


API_PROTOCOL = avro.protocol.Parse(
    pkg_resources.resource_string('mdcs_node_host', 'plugin.avpr'))


def serialize_value(schema, value):
    """
    Serialize a value to binary using the given Avro schema.
    """

    data_buffer = BytesIO()
    writer = DataFileWriter(data_buffer, DatumWriter(), schema)
    writer.append(value)
    writer.flush()

    return data_buffer.getvalue()


def unserialize_value(schema, data):
    """
    Unserialize a value from binary using the given Avro schema.
    """

    data_buffer = BytesIO(data)
    reader = DataFileReader(data_buffer, DatumReader())
    value = next(reader, None)
    reader.close()

    return value


class PluginResponder(Responder):
    def __init__(self, device):
        super().__init__(API_PROTOCOL)
        self.device = device

    def _get_time(self):
        return int(round(time.time() * 1000))

    def Invoke(self, message, request):
        if message.name == 'describe':
            attributes = []
            for path, attribute in self.device.attributes.items():
                flags = [flag.name for flag in attribute.flags]
                schema = json.dumps(attribute.schema.to_json())

                attributes.append({
                    'path': path,
                    'flags': flags,
                    'schema': schema
                })

            actions = []
            for path, action in self.device.actions.items():
                input_schema = json.dumps(action.input_schema.to_json())
                output_schema = json.dumps(action.output_schema.to_json())

                actions.append({
                    'path': path,
                    'input_schema': input_schema,
                    'output_schema': output_schema
                })

            return {
                'name': self.device.name,
                'attributes': attributes,
                'actions': actions
            }

        elif message.name == 'read' or message.name == 'write':
            # retrieve the attribute
            if request['path'] not in self.device.attributes:
                return {'message': 'attribute not found'}

            attribute = self.device.attributes[request['path']]

            # check if this is a read
            if message.name == 'read':
                # check if we can read the attribute
                if not attribute.readable:
                    return {
                        'message': 'attribute is not readable',
                        'attribute': attribute.path
                    }

                # read the value
                value = attribute.read()
                return {
                    'value': serialize_value(attribute.schema, value),
                    'time': self._get_time()}

            # check if we can write to the attribute
            if not attribute.writable:
                return {
                    'message': 'attribute is not writable',
                    'attribute': attribute.path
                }

            # write the value
            data = request['data']['value']
            value = unserialize_value(attribute.schema, data)
            attribute.write(value)

            # XXX client should be able to ask us to re-read the value and
            # return it (set-and-get)
            return {'value': data, 'time': self._get_time()}

        elif message.name == 'run':
            # retrieve the action
            if request['path'] not in self.device.actions:
                return {'message': 'action not found'}

            action = self.device.actions[request['path']]

            # run the action
            input_data = unserialize_value(
                action.input_schema,
                request['data']['value'])

            start_time = self._get_time()
            output_data = action.run(input_data)
            end_time = self._get_time()

            return {
                'value': serialize_value(action.output_schema, output_data),
                'start': start_time,
                'end': end_time
            }

        else:
            # unknown message
            raise AvroException("unexpected message: ", msg.name)

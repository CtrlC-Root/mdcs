import time
import socket
import select
import argparse

import psutil

from .device import Device, AttributeFlags, DelegatedAttribute
from .avro import PluginResponder


def read_memory_total():
    return psutil.virtual_memory().total


def read_memory_available():
    return psutil.virtual_memory().available


def read_memory_used():
    return psutil.virtual_memory().used


def read_memory_free():
    return psutil.virtual_memory().free


def main():
    # parse command line arguments
    parser = argparse.ArgumentParser()
    parser.add_argument('--host', default='127.0.0.1', help='node host')
    parser.add_argument('--port', type=int, required=True, help='node port')

    args = parser.parse_args()

    # create the host device
    device = Device("host-{}".format(socket.gethostname()))

    device.add_attribute(DelegatedAttribute(
        'memory.total',
        AttributeFlags.READ,
        'long',
        read_memory_total,
        None))

    device.add_attribute(DelegatedAttribute(
        'memory.available',
        AttributeFlags.READ,
        'long',
        read_memory_available,
        None))

    device.add_attribute(DelegatedAttribute(
        'memory.used',
        AttributeFlags.READ,
        'long',
        read_memory_used,
        None))

    device.add_attribute(DelegatedAttribute(
        'memory.free',
        AttributeFlags.READ,
        'long',
        read_memory_free,
        None))

    # connect to the node
    node_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    node_socket.connect((args.host, args.port))

    # create buffered files for reading and writing
    rfile = node_socket.makefile('rb', -1)
    wfile = node_socket.makefile('wb', -1)

    # create avro framed reader and writer
    reader = FramedReader(rfile)
    writer = FramedWriter(wfile)

    # processing loop
    responder = PluginResponder(device)
    while True:
        readable, writable, failed = select.select([node_socket], [], [])
        if node_socket in readable:
            request = reader.Read()
            print("request: {}".format(request))

            # TODO: handle control messages (i.e. quit)

            response = responder.Respond(request)
            print("response: {}".format(response))

            writer.Write(response)
            wfile.flush()

        else:
            time.sleep(0.1)

    # XXX cleanup
    wfile.flush()
    wfile.close()
    rfile.close()
    node_socket.close()

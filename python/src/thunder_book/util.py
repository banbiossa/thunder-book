import logging
import sys


def printerr(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)


def setup_logging():
    # console
    console_handler = logging.StreamHandler()
    console_handler.setLevel(logging.DEBUG)
    console_handler.setFormatter(
        logging.Formatter("%(asctime)s - %(filename)s - %(levelname)s - %(message)s")
    )

    # Create a logger for logging to output.md
    file_logger = logging.getLogger("file_logger")
    file_logger.setLevel(logging.DEBUG)

    # Create a file handler for output.md
    file_handler = logging.FileHandler("output.md")
    file_handler.setLevel(logging.INFO)
    file_handler.setFormatter(logging.Formatter("%(message)s"))

    # Add the file handler to the file logger
    file_logger.addHandler(file_handler)

    # Add the console handler to the file logger
    # file_logger.addHandler(console_handler)

    # Configure the root logger for logging to the console
    root_logger = logging.getLogger()
    root_logger.setLevel(logging.DEBUG)
    root_logger.addHandler(console_handler)

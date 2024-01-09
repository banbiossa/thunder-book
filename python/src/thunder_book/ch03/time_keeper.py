from datetime import datetime


class TimeKeeper:
    def __init__(self, time_threshold: int) -> None:
        """_summary_

        Args:
            time_threshold (int): threshold time in milliseconds
        """
        self.time_threshold = time_threshold * 1000
        self.start_time = datetime.now()

    def is_time_over(self) -> bool:
        diff = datetime.now() - self.start_time
        return diff.microseconds > self.time_threshold

import datetime

from freezegun import freeze_time

from thunder_book.ch03.time_keeper import TimeKeeper


def test_time_keeper():
    initial_datetime = datetime.datetime(
        year=1, month=7, day=12, hour=15, minute=6, second=3
    )
    with freeze_time(initial_datetime) as frozen_datetime:
        time_keeper = TimeKeeper(time_threshold=1)

        assert frozen_datetime() == initial_datetime
        assert not time_keeper.is_time_over()

        frozen_datetime.tick(delta=datetime.timedelta(microseconds=1))
        assert not time_keeper.is_time_over()

        frozen_datetime.tick(delta=datetime.timedelta(microseconds=1000))
        assert time_keeper.is_time_over()

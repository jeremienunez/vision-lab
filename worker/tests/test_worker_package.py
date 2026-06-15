import perception_worker


def test_worker_package_version() -> None:
    assert perception_worker.__version__ == "0.1.0"

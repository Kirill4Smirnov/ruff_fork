import platform
from platform import java_ver
from platform import java_ver as jv


platform.java_ver()
platform.java_ver("1.0")
java_ver()
jv(vendor="Acme")


class Wrapper:
    def java_ver(self):
        return ()


Wrapper().java_ver()
platform.python_version()

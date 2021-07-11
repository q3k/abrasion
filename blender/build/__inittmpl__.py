import json

bl_info = {
    'author': 'Abrasion Authors',
    'version': (1, 0, 0),
    'blender': (2, 80, 0),
    'name': "%name%",
    'location': "%location%",
    'category': "%category%",
}

import importlib
import os
import pathlib
import sys


_importpaths = json.loads("""%importpaths%""")
_this_path = str(pathlib.Path(__file__).parent.resolve())


def register():
    if _this_path not in sys.path:
        sys.path.append(_this_path)
    for p in _importpaths:
        fp = os.path.join(_this_path, p)
        if fp not in sys.path:
            sys.path.append(fp)

    import %module%
    %module% = importlib.reload(%module%)
    %module%.register()


def unregister():
    import %module%
    %module%.unregister()


if __name__ == '__main__':
    register()

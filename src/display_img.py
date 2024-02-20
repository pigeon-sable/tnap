#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
ローカルに保存された画像をターミナル上で表示する
"""

__author__ = 'Teruya Goda'
__version__ = '1.0.0'
__date__ = '2024/02/20 (Created: 2024/02/20)'

import subprocess
import os

def main():
    """
    main method
    """
    desktop_path = os.path.join(os.path.expanduser("~"), "Desktop")
    img_file = "cat.png"
    img_file_path = desktop_path + os.sep + img_file
    
    command = "imgcat " + img_file_path
    # command =  ["/usr/local/bin/imgcat", img_file_path]
    print(subprocess.run(command, shell=True))
    print(subprocess.run(f"\"bash -c {command} \"", shell=True))
    # subprocess.run(command, check=True)
    # print(command)
    # 結果（リターンコード：終了ステータス）を応答する。
    return 0



if __name__ == '__main__':
    # 上記のif文の記述によって、このスクリプトファイルが起動された時にだけ実行する部分になる。
    # $ python このスクリプトファイル名
    # と起動された時に__name__という変数に'__main__'という文字列が束縛されるゆえに。
    # つまり、このスクリプトがモジュールとしてインポートされた時には実行しないということ。

    # 実際にmain()を呼び出して、結果（リターンコード：終了ステータス）を得て、その結果でPythonシステムに終わりを告げる。
    import sys
    sys.exit(main())
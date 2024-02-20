#!/usr/bin/env python
# -*- coding: utf-8 -*-


"""
png画像からASCIIアートに変換するプログラム
現在は入力ファイルをハードコードで設定している(Desktopに存在していることが条件)
"""

__author__ = 'Teruya Goda'
__version__ = '1.0.0'
__date__ = '2024/02/19 (Created: 2024/02/19)'

from ascii_magic import AsciiArt
import os

def main():
    """
    main method
    
    """
    print(img2ascii())

    return 0

def img2ascii():
    """
    convert PNG to ASCII art.
    
    AsciiArt.to_ascii(
        columns: int = 120,
        width_ratio: float = 2.2,
        monochrome: bool = False,
        char: Optional[str],
        front: Optional[Front],
        back: Optional[Back]
    ) -> str        

    Returns:
        string: return ASCII art
    """
    
    my_art = AsciiArt.from_image(get_path())
    my_output = my_art.to_ascii(columns=169, monochrome=True)
    # my_test_output = my_art.to_html_file( 'my_test_output.html',columns=200, monochrome=True)
    
    return my_output
    
    

def get_path():
    """
    get path to the PNG file on the Desktop.
    
    Returns:
        string: return path
    """
    img_file = "milk.png"
    png_file_path = os.path.join(os.path.expanduser("~"), "Desktop", img_file)
    
    return  png_file_path


if __name__ == '__main__':
    # 上記のif文の記述によって、このスクリプトファイルが起動された時にだけ実行する部分になる。
    # $ python このスクリプトファイル名
    # と起動された時に__name__という変数に'__main__'という文字列が束縛されるゆえに。
    # つまり、このスクリプトがモジュールとしてインポートされた時には実行しないということ。

    # 実際にmain()を呼び出して、結果（リターンコード：終了ステータス）を得て、その結果でPythonシステムに終わりを告げる。
    import sys
    sys.exit(main())
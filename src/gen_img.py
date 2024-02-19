#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import argparse
import os
import sys

import requests
from dotenv import load_dotenv
from openai import OpenAI


def save_image(url, filename):
    """画像をURLからダウンロードし，ローカルに保存する"""
    response = requests.get(url)
    if response.status_code == 200:
        with open(filename, "wb") as f:
            f.write(response.content)
    else:
        print(f"Error: Failed to download the image from {url}")


def generate_image(prompt):
    """指定されたプロンプトに基づいて画像を生成し，そのURLを返す"""
    load_dotenv()
    OpenAI.api_key = os.getenv("OPENAI_API_KEY")
    client = OpenAI()

    response = client.images.generate(
        model="dall-e-3",
        prompt=prompt,
        size="1024x1024",
        quality="standard",
        n=1,
    )

    image_url = response.data[0].url
    return image_url


def main():
    """
    ユーザからプロンプトを受け取り，画像を生成して保存する
    """
    parser = argparse.ArgumentParser(
        description="Generate an image from a prompt using DALL-E 3."
    )
    parser.add_argument("prompt", type=str, help="Prompt for generating the image.")
    args = parser.parse_args()

    # 画像を生成
    image_url = generate_image(args.prompt)

    # 画像をローカルに保存
    filename = "generated_image.png"
    save_image(image_url, filename)
    print(f"Image saved as {filename}")


if __name__ == "__main__":
    sys.exit(main())

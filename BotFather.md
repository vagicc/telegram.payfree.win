otFather 命令

其余命令非常容易理解：

    /mybots – 返回您的机器人列表，其中包含方便的控件来编辑其设置。
    /mygames – 对您的游戏执行相同操作。

编辑机器人

要编辑您的机器人，您有两个选择。

您可以使用可用的命令：

    /setname – 更改你的机器人的名称。
    /setdescription – 更改机器人的描述（最多 512 个字符的短文本）。用户将在与机器人对话的开头看到此文本，标题为“这个机器人能做什么？ ”。
    /setabouttext – 更改机器人的关于信息，一段最多 120 个字符的较短文本。用户将在机器人的个人资料页面上看到此文本。当他们与他人分享您的机器人时，此文本将与链接一起发送。
    /setuserpic – 更改机器人的个人资料图片。
    /setcommands – 更改您的机器人支持的命令列表。用户在/与您的机器人聊天时输入这些命令时，将以建议的形式看到这些命令。有关更多信息，请参阅命令。
    /setdomain – 将网站域名链接到您的机器人。请参阅登录小部件部分。
    /deletebot – 删除您的机器人并释放其用户名。无法撤消。

或者您可以使用/mybots命令，点击您的机器人并使用现代内联界面来编辑它。

    从 2023 年 4 月 21 日 ( Telegram 9.6 ) 开始，您可以直接从机器人的个人资料页面编辑其面向公众的信息 - 包括设置自定义个人资料视频。

编辑设置

    /setinline – 为您的机器人切换内联模式。
    /setinlinegeo – 请求位置数据以提供基于位置的内联结果。
    /setjoingroups – 切换您的机器人是否可以添加到群组。所有机器人都必须能够处理直接消息，但如果您的机器人不是为群组工作而设计的，您可以禁用此功能。
    /setinlinefeedback – 切换 API 是否应发送有关用户选择的结果的更新。请参阅此处的详细说明。
    /setprivacy – 设置您的机器人在添加到群组时将收到哪些消息。有关更多信息，请参阅隐私模式。

管理游戏

    /newgame—创建新游戏。
    /listgames – 查看您的游戏列表。
    /editgame – 编辑游戏。
    /deletegame – 删除现有游戏。

    请注意，更改可能需要几分钟才能生效。

有了这些信息，您就可以继续阅读我们的面向开发人员的完整 API 参考。

    如果您有任何疑问，请查看我们的机器人常见问题解答。
    如果您在使用我们的 API 时遇到问题，请联系Telegram 上的@BotSupport 。

    # =====================
    新建机器在人：
    /newbot


Alright, a new bot. How are we going to call it? Please choose a name for your bot.
好的，一个新机器人。我们该怎么称呼它？请为您的机器人选择一个名称。

    开发测试机器人
Good. Now let's choose a username for your bot. It must end in `bot`. Like this, for example: TetrisBot or tetris_bot.

xustTestBot


Done! Congratulations on your new bot. You will find it at t.me/xustTestBot. You can now add a description, about section and profile picture for your bot, see /help for a list of commands. By the way, when you've finished creating your cool bot, ping our Bot Support if you want a better username for it. Just make sure the bot is fully operational before you do this.

Use this token to access the HTTP API:
7666999814:AAGamwcDTveGfRFIqOrqP06no-_mWaQk2Gg
Keep your token secure and store it safely, it can be used by anyone to control your bot.

For a description of the Bot API, see this page: https://core.telegram.org/bots/api



================================================================
BotFather创建机器人
/newbot
Alright, a new bot. How are we going to call it? Please choose a name for your bot.
临来笑笑生
Good. Now let's choose a username for your bot. It must end in `bot`. Like this, for example: TetrisBot or tetris_bot.
xustLuckBot

Done! Congratulations on your new bot. You will find it at t.me/xustLuckBot. You can now add a description, about section and profile picture for your bot, see /help for a list of commands. By the way, when you've finished creating your cool bot, ping our Bot Support if you want a better username for it. Just make sure the bot is fully operational before you do this.

Use this token to access the HTTP API:
7921154296:AAH-h1wl6v_uNig26FgoLIqE64CJqUATOwA
Keep your token secure and store it safely, it can be used by anyone to control your bot.

For a description of the Bot API, see this page: https://core.telegram.org/bots/api
================================================================
// 微信接口返回码，全局返回码请参考 https://mp.weixin.qq.com/wiki?id=mp1433747234

// 系统错误
pub const SYSTEM_ERROR: i32 = -1000;

// 系统繁忙
// 此时请开发者稍候再试
pub const SYSTEM_BUSY: i32 = -1;

// 请求成功
pub const SUCCESS: i32 = 0;

// AppSecret 错误，或是 Access Token 无效
// 请开发者认真比对AppSecret的正确性，或查看是否正在为恰当的公众号调用接口
pub const INVALID_CREDENTIAL: i32 = 40001;

// 错误的凭证类型
pub const INVALID_CREDENTIAL_TYPE: i32 = 40002;

// 错误的 OpenID
// 请开发者确认 OpenID 是否已关注公众号，或是否是其他公众号的 OpenID
pub const INVALID_OPENID: i32 = 40003;

// 不支持的媒体文件类型
pub const INVALID_MEDIA_TYPE: i32 = 40004;

// 不支持的文件类型
pub const INVALID_FILE_TYPE: i32 = 40005;

// 不支持的文件大小
pub const INVALID_FILE_SIZE: i32 = 40006;

// 错误的 MediaID
pub const INVALID_MEDIA_ID: i32 = 40007;

// 错误的消息类型
pub const INVALID_MESSAGE_TYPE: i32 = 40008;

// 不支持的图片大小
// 图片格式不对有时也会报这个错
pub const INVALID_IMAGE_SIZE: i32 = 40009;

// 不支持的语音文件大小
pub const INVALID_VOICE_SIZE: i32 = 40010;

// 不支持的视频文件大小
pub const INVALID_VIDEO_SIZE: i32 = 40011;

// 不支持的缩略图大小
pub const INVALID_THUMB_SIZE: i32 = 40012;

// 错误的 AppID
// 目前 AppID 格式都是 /^wx\d{16}$/
pub const INVALID_APP_ID: i32 = 40013;

// 不合法的 Access Token
// 请开发者认真比对 Access Token 的有效性（如是否过期），或查看是否正在为恰当的公众号调用接口
pub const INVALID_ACCESS_TOKEN: i32 = 40014;

// 错误的按钮类型
pub const INVALID_BUTTON_TYPE: i32 = 40015;

// 不支持的主菜单按钮个数
// 微信自定义菜单按钮个数应该在 1~3 个之间
pub const INVALID_BUTTON_SIZE: i32 = 40016;

// 不支持的子菜单按钮个数
// 微信自定义子菜单按钮个数应该在 1~5 个之间
pub const INVALID_SUB_BUTTON_SIZE: i32 = 40017;

// 不支持的按钮名字长度
pub const INVALID_BUTTON_NAME_SIZE: i32 = 40018;

// 不支持的按钮 key 长度
pub const INVALID_BUTTON_KEY_SIZE: i32 = 40019;

// 不支持的按钮 url 长度
pub const INVALID_BUTTON_URL_SIZE: i32 = 40020;

// 不合法的菜单版本号
pub const INVALID_MENU_VERSION: i32 = 40021;

// 不合法的子菜单级数
pub const INVALID_SUB_BUTTON_LEVEL: i32 = 40022;

// 不合法的子菜单按钮个数
pub const INVALID_SUB_BUTTON_COUNT: i32 = 40023;

// 不合法的子菜单按钮类型
pub const INVALID_SUB_BUTTON_TYPE: i32 = 40024;

// 不合法的子菜单按钮名字长度
pub const INVALID_SUB_BUTTON_NAME_SIZE: i32 = 40025;

// 不合法的子菜单按钮 key 长度
pub const INVALID_SUB_BUTTON_KEY_SIZE: i32 = 40026;

// 不合法的子菜单按钮 url 长度
pub const INVALID_SUB_BUTTON_URL_SIZE: i32 = 40027;

// 不合法的自定义菜单使用用户
pub const INVALID_MENU_USER: i32 = 40028;

// 错误的 OAuth Code
pub const INVALID_OAUTH_CODE: i32 = 40029;

// 错误的 Refresh Token
pub const INVALID_REFRESH_TOKEN: i32 = 40030;

// 错误的 OpenID 列表
pub const INVALID_OPENID_LIST: i32 = 40031;

// 错误的 OpenID 列表长度
// 列表内最多10000个 OpenID
pub const INVALID_OPENID_LIST_SIZE: i32 = 40032;

// 不支持的请求字符
// 不能包含 \uxxxx 格式的字符
pub const INVALID_REQUEST_CHARSET: i32 = 40033;

// 不合法的参数
pub const INVALID_PARAMETER: i32 = 40035;

// 错误的模板消息 ID
// Template ID 失效了，请重新刷新一次 Template ID
pub const INVALID_TEMPLATE: i32 = 40037;

// 不合法的请求格式
pub const INVALID_REQUEST_FORMAT: i32 = 40038;

// 不合法的 url 长度
pub const INVALID_URL_SIZE: i32 = 40039;

// 不合法的分组 ID
pub const INVALID_GROUP_ID: i32 = 40050;

// 不合法的分组名字
pub const INVALID_GROUP_NAME: i32 = 40051;

// 不支持的操作
// 可能是该公众号已经申请完了十万个二维码
pub const INVALID_ACTION_INFO: i32 = 40053;

// 自定义菜单的按钮里，网址有误
pub const INVALID_BUTTON_DOMAIN: i32 = 40054;

// 自定义子菜单的按钮里，网址有误
pub const INVALID_SUB_BUTTON_DOMAIN: i32 = 40055;

// 错误的图文消息 ID
pub const INVALID_ARTICLE_ID: i32 = 40060;

// 错误的行业号
// 有一些模板消息只会在特定的行业下申请
pub const INVALID_INDUSTRY_ID: i32 = 40102;

// 不支持的 MediaID 长度
pub const INVALID_MEDIA_ID_SIZE: i32 = 40118;

// 不支持的 MediaID 类型
pub const INVALID_MEDIA_ID_TYPE: i32 = 40121;

// 缺少 Access Token 参数
pub const MISSING_ACCESS_TOKEN: i32 = 41001;

// 缺少 AppID 参数
pub const MISSING_APP_ID: i32 = 41002;

// 缺少 Refresh Token 参数
pub const MISSING_REFRESH_TOKEN: i32 = 41003;

// 缺少 AppSecret 参数
pub const MISSING_APP_SECRET: i32 = 41004;

// 缺少多媒体文件数据
pub const MISSING_MEDIA_DATA: i32 = 41005;

// 缺少 MediaID 参数
pub const MISSING_MEDIA_ID: i32 = 41006;

// 缺少子菜单数据
pub const MISSING_SUB_BUTTONS: i32 = 41007;

// 缺少 OAuth Code
pub const MISSING_OAUTH_CODE: i32 = 41008;

// 缺少 OpenID
pub const MISSING_OPENID: i32 = 41009;

// Access Token 已失效
// 请检查 Access Token 的有效期，重新刷新 Access Token
pub const EXPIRED_ACCESS_TOKEN: i32 = 42001;

// Refresh Token 已失效
pub const EXPIRED_REFRESH_TOKEN: i32 = 42002;

// OAuth Code 已失效
pub const EXPIRED_OAUTH_CODE: i32 = 42003;

// 授权已失效
// 用户修改微信密码，Access Token, Refresh Token 均已失效，需要重新授权
pub const EXPIRED_AUTHORIZATION: i32 = 42007;

// 需要 Get 请求
pub const REQUIRE_GET: i32 = 43001;

// 需要 Post 请求
pub const REQUIRE_POST: i32 = 43002;

// 需要 Https 请求
pub const REQUIRE_HTTPS: i32 = 43003;

// 用户没有关注公众号
pub const REQUIRE_SUBSCRIBE: i32 = 43004;

// 用户被拉黑
// 需要公众号把该用户从黑名单里移除
pub const REQUIRE_UNBLOCK_USER: i32 = 43019;

// 超过了更换行业的限制
// 一个月最多换一次
pub const OUT_OF_CHANGE_INDUSTRY_LIMIT: i32 = 43100;

// 多媒体文件大小超过限制
// 最大允许 1MB
pub const OUT_OF_MEDIA_SIZE_LIMIT: i32 = 45001;

// 消息内容超过限制
pub const OUT_OF_CONTENT_SIZE_LIMIT: i32 = 45002;

// 标题长度超过限制
// 最长允许 64 字符长度
pub const OUT_OF_TITLE_SIZE_LIMIT: i32 = 45003;

// 描述字段超过限制
pub const OUT_OF_DESCRIPTION_SIZE_LIMIT: i32 = 45004;

// 链接字段超过限制
pub const OUT_OF_URL_SIZE_LIMIT: i32 = 45005;

// 图片链接字段超过限制
pub const OUT_OF_PIC_URL_SIZE_LIMIT: i32 = 45006;

// 语音播放时间超过限制
// 最长允许 60 秒
pub const OUT_OF_VOICE_TIME_LIMIT: i32 = 45007;

// 图文消息数量超过限制
// 最多 10 条图文消息
pub const OUT_OF_ARTICLE_SIZE_LIMIT: i32 = 45008;

// 接口调用频率超过限制
pub const OUT_OF_API_FREQ_LIMIT: i32 = 45009;

// 回复时间超过限制
// 接受推送后，5 秒内未被动响应。或者是用户与公众号 48 小时无互动后，调用客服接口主动推送消息。
pub const OUT_OF_RESPONSE_TIME_LIMIT: i32 = 45015;

// 模板消息数量超过限制
pub const OUT_OF_TEMPLATE_SIZE_LIMIT: i32 = 45026;

// 模板消息与行业信息冲突
pub const TEMPLATE_CONFLICT_WITH_INDUSTRY: i32 = 45027;

// 不支持的图文消息内容
// 请确认 content 里没有超链接标签
pub const INVALID_CONTENT: i32 = 45166;

// API 功能未授权
// 请确认公众号已获得该接口，可以在公众平台官网-开发者中心页中查看接口权限
pub const UNAUTHORIZED_API: i32 = 48001;

// 用户拒收公众号消息
// (在公众号选项中，关闭了“接收消息”)
pub const USER_BLOCK_MESSAGE: i32 = 48002;

// 公众号管理员没有同意微信群发协议
// 请登录公众号后台点一下同意
pub const USER_NOT_AGREE_PROTOCOL: i32 = 48003;

// API 接口被封禁
// 请登录公众号后台查看详情
pub const API_BANNED: i32 = 48004;

// API 禁止删除被自动回复和自定义菜单引用的素材
pub const API_DELETE_PROHIBITED: i32 = 48005;

// API 清零次数失败，因为清零次数达到上限
pub const OUT_OF_RESET_LIMIT: i32 = 48006;

// 公众号未授权给开放平台
pub const UNAUTHORIZED_COMPONENT: i32 = 61003;

// 公众号未授权该 API 给开放平台
pub const UNAUTHORIZED_COMPONENT_API: i32 = 61007;

// 错误的开放平台 Refresh Token
pub const INVALID_COMPONENT_REFRESH_TOKEN: i32 = 61023;

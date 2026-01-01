# 需求描述

提供一个支持SQL事务的插件。支持SQLite、MySQL、PostgreSQL数据库。

## 插件名称

tauri-plugin-sql-transaction

## 插件版本

1.0.0

## 插件类型

Tauri

## 插件语言

Rust
TypeScript

## 插件接口

参考tauri-plugin-sqlite

## 实现逻辑

事务接口的具体实现逻辑可参考tauri-plugin-sqlite

## npm依赖

@tauri-apps/plugin-sql 该依赖中提供了Tauri加载数据库的接口。前端API接口需直接使用该依赖中的接口，可对接口进行扩展。

## rust依赖
tauri-plugin-sql 该依赖已经实现的数据库接口，rust中可以直接使用，并对其进行扩展。

## 测试

1. 提供必要的单元测试，并执行通过。
2. 提供必要的集成测试，并执行通过。
3. 在examples提供必要的示例代码。并执行通过。

## 发布

1. 提供必要的发布文档。
2. 提供必要的发布脚本。
3. 提供必要的发布流程。

## 额外要求

1. 对TypeScript接口提供类型定义。并优化接口设计，提供更友好的API。
2. 对Rust接口提供文档。检视代码实现逻辑，采用最佳实践优化代码结构，确保代码质量，提升代码可读性。
3. 持续进行深度优化，不断提升代码质量，提供更优质的生产可用的插件。
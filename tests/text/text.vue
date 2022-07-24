<template>
  <div class="header-container">
    <span class="logo">UA平台</span>
    <div class="nav-container">
      <div
        class="nav-item"
        v-for="item in config"
        :key="item.path"
        :class="{ active: curPath.indexOf(item.path) === 0 }"
      >
        <a v-if="!item.type" :href="item.path" :target="item.target">{{ item.title }}</a>
        <mtd-dropdown v-else trigger="hover">
          <mtd-button type="text">
            <a style="margin-right: 4px;">{{ item.title }} 梦想</a>
            <i class="mtdicon mtdicon-down"></i>
          </mtd-button>
          <mtd-dropdown-menu slot="dropdown">
            <mtd-dropdown-menu-item
              class="dropdown-link-container"
              v-for="sub of item.sub"
              :key="sub.path"
            >
              <a class="dropdown-link" :href="sub.path" :key="sub.path" :target="sub.target"> 卡片{{
                sub.title
              }}</a>
            </mtd-dropdown-menu-item>
          </mtd-dropdown-menu>
        </mtd-dropdown>
      </div>
    </div>
    <div class="nav-help">
      <a href="https://km.sankuai.com/page/178921778" target="_blank">使用手册</a>
      <mtd-popover trigger="hover" class="item" placement="bottom">
        <a href="javascript:void(0);">交流群</a>
        <div slot="content" class="demo-popover-content">
          <img src="../../assets/code.png" />
          <div>大象扫一扫</div>
        </div>
      </mtd-popover>
      <a
        href="https://tt.sankuai.com/ticket/create?category=%E5%A4%A7%E6%95%B0%E6%8D%AE%E4%B8%8E%E7%AE%97%E6%B3%95&type=%E6%95%B0%E6%8D%AE%E4%BA%A7%E5%93%81&item=UA"
        target="_blank"
        >反馈</a
      >
    </div>
    <div class="right-container">
      <i class="mtdicon-xx"></i>
      <i class="mtdicon-xx"></i>
      <UserInfo></UserInfo>
    </div>
  </div>
</template>

<script lang="ts">
import Vue from 'vue'
import { Component } from 'vue-property-decorator'
import UserInfo from './UserInfo.vue'
import config from './headerConfig'

@Component({
  components: {
    UserInfo,
  },
})
export default class HeaderNav extends Vue {
  config = config
  activeTab = '/main/#/'

  get curPath() {
    return `/main/#/${this.$route.path}`
  }
}
</script>
<style lang="scss" scoped>
.demo-popover-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  img {
    width: 100px;
    height: 100px;
    margin-bottom: 4px;
  }
}
.dropdown-link {
  color: #6f6f6f;
  font-weight: unset !important;
  text-decoration: none;
  &-container {
    text-align: center;
  }
  &:hover {
    background-color: #edf0f7;
    color: #464646;
  }
}
.header-container {
  width: 100%;
  height: 56px;
  background: #293454;
  box-shadow: 0 1px 1px 0 rgba(21, 29, 50, 0.25);
  text-align: left;
  display: flex;
  flex-direction: row;
  align-items: center;
  .logo {
    margin-left: 36px;
    line-height: 56px;
    font-family: PingFangSC-Regular;
    font-size: 20px;
    color: #ffffff;
  }

  .nav-container {
    display: inline-block;
    margin-left: 60px;
    font-family: PingFangSC-Regular;
    font-size: 14px;
    flex: 1;
    & .nav-item {
      display: inline-block;
      margin-right: 50px;
      cursor: pointer;

      a {
        color: #c0c4e0 !important;
        font-weight: unset !important;
        text-decoration: none;
      }

      &.active,
      &:hover {
        font-family: PingFangSC-Medium;
        a {
          color: #fff !important;
        }
      }
    }
  }
  .nav-help {
    display: flex;
    flex-direction: row;
    align-items: center;
    height: 100%;
    a {
      color: #c0c4e0 !important;
      font-weight: unset !important;
      text-decoration: none;
      margin: 0 10px;
      &.active,
      &:hover {
        color: #fff !important;
      }
    }
  }
  .right-container {
    margin: 0 24px 0 10px;
    color: #fff;
    .busitheme-line {
      margin: 12px 104px 12px 0;
    }
  }
}
</style>

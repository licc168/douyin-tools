<template>
  <div class="common-layout">
    <el-container>

      <el-header>

        <div>


          <el-popover :visible="visible" offset="20" placement="right" :width="800" trigger="click">
            <template #reference>
              <el-button @click="setting()" type="success" style="margin-bottom: 10px">设置</el-button>
            </template>
              <el-form-item label="csv路径:">

                <el-input
                    v-model="filePath"
                    :placeholder="pathPlaceholder"
                    type="text"
                >
                </el-input>
              </el-form-item>
              <el-form-item label="cookie:">
                <el-input
                    v-model="userCookie"
                    placeholder="设置Cookie"
                    type="textarea"
                    :autosize="{ minRows: 8, maxRows: 10 }"
                >
                </el-input>
                <div style="text-align: right; margin: 0">
                  <el-button size="small" type="primary" @click="visible = false"
                  >保存
                  </el-button
                  >
                </div>
              </el-form-item>

          </el-popover>
          <div class="textarea-button-wrapper">

            <el-input
                v-model="video_urls"
                placeholder="请输入抖音视频链接：如https://www.douyin.com/video/7259237588194659638 多个换行"
                type="textarea"
                :autosize="{ minRows: 3, maxRows: 5 }"
            >
            </el-input>

            <el-button
                class="inner-button"
                type="success"
                @click="getData(video_urls)"

                round
                :disabled="isButtonDisabled"

            >
              开始采集
            </el-button>
          </div>
        </div>

      </el-header>
      <el-main>
        <div>
          <!--          <el-button @click="reply()" type="success" style="margin-bottom: 10px">批量回复</el-button>-->
        </div>
        <el-table :data="tableData">
          <el-table-column type="selection" width="55"/>

          <el-table-column prop="id" label="序号"/>
          <el-table-column prop="link" label="目标链接" width="180"/>
          <el-table-column prop="name" label="评论者"/>
          <el-table-column prop="main_link" label="主页链接"/>
          <el-table-column prop="date" label="评论时间"/>

          <el-table-column prop="address" label="ip属地"/>

          <el-table-column prop="comment_count" label="评论点赞数"/>

          <el-table-column prop="comment_level" label="评论级别"/>

          <el-table-column prop="comment_content" label="评论内容"/>

        </el-table>


      </el-main>
      <div><h4>日志信息</h4></div>

      <el-footer>

        <div v-html="log"></div>
      </el-footer
      >
    </el-container>
  </div>
</template>

<script lang="ts" setup>
import {invoke} from "@tauri-apps/api/tauri";
import {ref} from 'vue';
import {listen} from "@tauri-apps/api/event";
import {downloadDir} from '@tauri-apps/api/path';


type TableRowData = {
  id: number,
  link: string;
  main_link: string;
  name: string;
  date: string;
  address: string;
  comment_count: number;
  comment_level: string;
  comment_content: string;
};
// 使用泛型定义一个包含 TableRowData 类型的数组类型
type TableRowDataArray = TableRowData[];
const video_urls = ref('https://www.douyin.com/video/7341701130524658996');
const userCookie = ref('');
const isButtonDisabled = ref(false);
const visible = ref(false)

const log = ref('');
const filePath = ref('');
const pathPlaceholder = ref('设置导出 csv 路径，不填则默认下载路径');


const tableData = ref<TableRowDataArray>([]);


interface ResponseData {
  code: number;
  msg: string;
  data: any[];
}

async function setting() {
  visible.value = true
  const downloadDirPath = await downloadDir();
  pathPlaceholder.value = "不填则默认" + downloadDirPath;

}

function reply() {
  ElMessage({
    showClose: true,
    message: "功能待完善",
    type: 'info',
  })
}

async function getData(videoUrls: String) {
  isButtonDisabled.value = true;
  if (filePath.value.length == 0) {
    filePath.value = await downloadDir()
  }
  if (userCookie.value.length == 0) {
    ElMessage({
      showClose: true,
      message: "cookie 不能为空",
      type: 'error',
    })
    return
  }
  console.log(videoUrls);
  let path =
      invoke("collect_comment", {
        filePath: filePath.value,
        cookie: userCookie.value,
        videoUrls: videoUrls
      }).then((data) => {
        const responseData = data as ResponseData; // Type assertion here

        if (responseData.code === 200) {

        } else {
          ElMessage({
            showClose: true,
            message: responseData.msg,
            type: 'error',
          })
        }
      })

}

listen("event_data", (event) => {
  console.log(event.payload)
  const payload = event.payload as TableRowData;
  console.log(payload)
  tableData.value.push(payload);
});
listen("event_log", (event) => {
  console.log(event.payload)
  const payload = event.payload as String + "</br>";
  if (payload.includes("采集完成")) {
    isButtonDisabled.value = false;
    ElMessage({
      showClose: true,
      message: "采集完成",
      type: 'success',
    })
  }
  log.value += payload;

});
</script>

<style scoped>
.common-layout {
  height: 95vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;

}

.el-header {
  display: flex;

  flex-direction: column;
}

.el-main {
  display: flex;

  flex-direction: column;
  overflow: auto;
  height: 300px;
  margin-top: 50px;
}

.el-footer {
  flex-shrink: 0;
  margin-bottom: 1px;
  overflow: auto;
  height: 200px;
  background-color: antiquewhite;
}


/** 对话框 */


/** 文本域 按钮 */
.textarea-button-wrapper {
  position: relative;
}


.inner-button {
  position: absolute;
  right: 10px;
  bottom: 10px;
}
</style>


  

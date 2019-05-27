import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequest(findTestObject('Server/GetServer'))

WS.verifyResponseStatusCode(response, 200)

WS.verifyElementPropertyValue(response, 'data[0].id', 7)
WS.verifyElementPropertyValue(response, 'data[0].ipaddress', '213.112.2.2')
WS.verifyElementPropertyValue(response, 'data[0].serverCode', '5812KIQF')
WS.verifyElementPropertyValue(response, 'data[0].companyId', 3)
WS.verifyElementPropertyValue(response, 'data[0].createDate', '2019-05-06T16:06:23.407')
WS.verifyElementPropertyValue(response, 'data[0].description', 'Test')
WS.verifyElementPropertyValue(response, 'data[0].active', 'true')

WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].id', 20)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].createTime', '2019-05-06T16:07:36.6')
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].lastUpdate', '2019-05-23T14:20:50.323')
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].name', 'Test System Instance')
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].status', 1)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].description', 'Ok')
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].status', 1)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].serverId', 7)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].active', 'true')
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].status', 1)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[0].systemsId', 16)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[1].id', 21)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[1].createTime', '2019-05-06T16:08:30.243')
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[1].lastUpdate', '2019-05-23T14:20:50.323')

WS.verifyElementPropertyValue(response, 'data[0].systemInstance[1].name', 'Test System Instance 2')
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[1].status', 1)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[1].description', 'Ok')
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[1].serverId', 7)
WS.verifyElementPropertyValue(response, 'data[0].systemInstance[1].systemsId', 16)



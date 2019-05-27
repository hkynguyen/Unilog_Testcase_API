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

getCompany = WS.sendRequest(findTestObject('Company/GetCompany'))

WS.verifyResponseStatusCode(getCompany, 200)

WS.verifyElementPropertyValue(getCompany, 'data[0].name', 'Wisky Company')

WS.verifyElementPropertyValue(getCompany, 'data[0].id', 5)

WS.verifyElementPropertyValue(getCompany, 'data[0].phone', '0123456789')

WS.verifyElementPropertyValue(getCompany, 'data[0].address', 'QTSC Incubator')

WS.verifyElementPropertyValue(getCompany, 'data[0].createTime', '2019-04-04T00:00:00')

WS.verifyElementPropertyValue(getCompany, 'data[0].active', 'true')

WS.verifyElementPropertyValue(getCompany, 'data[0].activeCode', 'Actived')


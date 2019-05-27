<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PostRegisterEmployee</name>
   <tag></tag>
   <elementGuidId>3f4663a6-5207-4e54-89ef-9ec6070bc743</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;yumi.nguyen@gmail.com\&quot;,\n  \&quot;password\&quot;: \&quot;zaQ@1234\&quot;,\n  \&quot;confirmPassword\&quot;: \&quot;zaQ@1234\&quot;,\n  \&quot;companyId\&quot;: 4,\n  \&quot;regisAdminToken\&quot;: \&quot;Tok3n!2019\&quot;,\n  \&quot;registerCode\&quot;: \&quot;string\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1laWQiOiIyIiwidW5pcXVlX25hbWUiOiJpYS50ZWFtLmsxMkBnbWFpbC5jb20iLCJyb2xlIjpbIkFjdGl2ZVVzZXIiLCJNYW5hZ2VyIl0sIm5iZiI6MTU1ODk0ODEyMywiZXhwIjoxNTY0MTU3MzIzLCJpYXQiOjE1NTg5NDgxMjMsImlzcyI6Imh0dHBzOi8vbG9jYWxob3N0OjQ0MzI4IiwiYXVkIjoiaHR0cHM6Ly9sb2NhbGhvc3Q6NDQzMjgifQ.YEcQA3lqkmDc5sNQ5uDMYma5eqeZiRYUgBipNnq92VQ</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://apiunilog.unicode.edu.vn/api/Register/employee?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

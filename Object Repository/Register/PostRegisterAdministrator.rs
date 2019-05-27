<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PostRegisterAdministrator</name>
   <tag></tag>
   <elementGuidId>361c8908-35b0-43ed-80ff-14712f437b13</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;email\&quot;: \&quot;hky.nguyen@wisky.vn\&quot;,\n  \&quot;password\&quot;: \&quot;zaQ@1234\&quot;,\n  \&quot;confirmPassword\&quot;: \&quot;zaQ@1234\&quot;,\n  \&quot;companyId\&quot;: 3,\n  \&quot;regisAdminToken\&quot;: \&quot;Tok3n!2019\&quot;,\n  \&quot;registerCode\&quot;: \&quot;string\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJuYW1laWQiOiIxIiwidW5pcXVlX25hbWUiOiJ3aXNreS5zeXN0ZW1tb25pdG9yQGdtYWlsLmNvbSIsInJvbGUiOlsiQWN0aXZlVXNlciIsIkFkbWluaXN0cmF0b3IiXSwibmJmIjoxNTU4OTQzMTI2LCJleHAiOjE1NjQxNTIzMjYsImlhdCI6MTU1ODk0MzEyNiwiaXNzIjoiaHR0cHM6Ly9sb2NhbGhvc3Q6NDQzMjgiLCJhdWQiOiJodHRwczovL2xvY2FsaG9zdDo0NDMyOCJ9.mR5FQnS7YlFPdByQaDsCvWDcXTj6raO4JBEgvNlZl5s</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://apiunilog.unicode.edu.vn/api/Register/administrator?</restUrl>
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
